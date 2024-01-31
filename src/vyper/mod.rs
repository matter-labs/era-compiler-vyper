//!
//! The Vyper compiler.
//!

pub mod combined_json;
pub mod standard_json;
pub mod version;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use sha3::digest::FixedOutput;
use sha3::Digest;

use crate::project::contract::vyper::Contract as VyperContract;
use crate::project::contract::Contract;
use crate::project::Project;

use self::combined_json::CombinedJson;
use self::standard_json::input::Input as StandardJsonInput;
use self::standard_json::output::Output as StandardJsonOutput;
use self::version::Version;

///
/// The Vyper compiler.
///
pub struct Compiler {
    /// The binary executable name.
    pub executable: String,
    /// The binary version.
    pub version: Version,
}

impl Compiler {
    /// The default executable name.
    pub const DEFAULT_EXECUTABLE_NAME: &'static str = "vyper";

    /// The supported versions of `vyper`.
    pub const SUPPORTED_VERSIONS: [semver::Version; 3] = [
        semver::Version::new(0, 3, 3),
        semver::Version::new(0, 3, 9),
        semver::Version::new(0, 3, 10),
    ];

    ///
    /// A shortcut constructor.
    ///
    /// Different tools may use different `executable` names. For example, the integration tester
    /// uses `vyper-<version>` format.
    ///
    pub fn new(executable: String) -> anyhow::Result<Self> {
        if let Err(error) = which::which(executable.as_str()) {
            anyhow::bail!(
                "The `{executable}` executable not found in ${{PATH}}: {}",
                error
            );
        }
        let version = Self::version(&executable)?;
        Ok(Self {
            executable,
            version,
        })
    }

    ///
    /// The `vyper -f combined_json input_files...` mirror.
    ///
    pub fn combined_json(
        &self,
        paths: &[PathBuf],
        evm_version: Option<compiler_llvm_context::EVMVersion>,
    ) -> anyhow::Result<CombinedJson> {
        let mut command = std::process::Command::new(self.executable.as_str());
        if let Some(evm_version) = evm_version {
            command.arg("--evm-version");
            command.arg(evm_version.to_string());
        }
        command.arg("-f");
        command.arg("combined_json");
        command.args(paths);
        if self.version.default >= semver::Version::new(0, 3, 10) {
            command.arg("--no-optimize");
        }
        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let mut combined_json: CombinedJson =
            compiler_common::deserialize_from_slice(output.stdout.as_slice()).map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess output parsing error: {}\n{}",
                    self.executable,
                    error,
                    compiler_common::deserialize_from_slice::<serde_json::Value>(
                        output.stdout.as_slice()
                    )
                    .map(|json| serde_json::to_string_pretty(&json).expect("Always valid"))
                    .unwrap_or_else(
                        |_| String::from_utf8_lossy(output.stdout.as_slice()).to_string()
                    ),
                )
            })?;
        combined_json.remove_evm();
        Ok(combined_json)
    }

    ///
    /// The `vyper --standard-json` mirror.
    ///
    pub fn standard_json(
        &self,
        mut input: StandardJsonInput,
    ) -> anyhow::Result<StandardJsonOutput> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.arg("--standard-json");

        if self.version.default >= semver::Version::new(0, 3, 10) {
            input.settings.optimize = false;
        }
        let input_json = serde_json::to_vec(&input).expect("Always valid");

        let process = command.spawn().map_err(|error| {
            anyhow::anyhow!("{} subprocess spawning error: {:?}", self.executable, error)
        })?;
        process
            .stdin
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("{} stdin getting error", self.executable))?
            .write_all(input_json.as_slice())
            .map_err(|error| {
                anyhow::anyhow!("{} stdin writing error: {:?}", self.executable, error)
            })?;

        let output = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!("{} subprocess output error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let mut output: StandardJsonOutput =
            compiler_common::deserialize_from_slice(output.stdout.as_slice()).map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess output parsing error: {}\n{}",
                    self.executable,
                    error,
                    compiler_common::deserialize_from_slice::<serde_json::Value>(
                        output.stdout.as_slice()
                    )
                    .map(|json| serde_json::to_string_pretty(&json).expect("Always valid"))
                    .unwrap_or_else(
                        |_| String::from_utf8_lossy(output.stdout.as_slice()).to_string()
                    ),
                )
            })?;

        for (full_path, source) in input.sources.into_iter() {
            let last_slash_position = full_path.rfind('/');
            let last_dot_position = full_path.rfind('.');
            let contract_name = &full_path[last_slash_position.unwrap_or_default()
                ..last_dot_position.unwrap_or(full_path.len())];

            Self::check_unsupported(source.content.as_str())
                .map_err(|error| anyhow::anyhow!("Contract `{}`: {}", full_path, error))?;

            output
                .contracts
                .as_mut()
                .ok_or_else(|| {
                    anyhow::anyhow!(serde_json::to_string(&output.errors).expect("Always valid"))
                })?
                .get_mut(full_path.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!("File `{}` not found in the standard JSON output", full_path)
                })?
                .get_mut(contract_name)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Contract `{}` not found in the standard JSON output",
                        contract_name
                    )
                })?
                .source_code = Some(source.content);
        }

        Ok(output)
    }

    ///
    /// Returns the Vyper LLL in the native format for the contract at `path`.
    ///
    /// Is used to print the IR for debugging.
    ///
    pub fn lll_debug(
        &self,
        path: &Path,
        evm_version: Option<compiler_llvm_context::EVMVersion>,
        optimize: bool,
    ) -> anyhow::Result<String> {
        let mut command = std::process::Command::new(self.executable.as_str());
        if let Some(evm_version) = evm_version {
            command.arg("--evm-version");
            command.arg(evm_version.to_string());
        }
        command.arg("-f");
        command.arg("ir");
        if !optimize || self.version.default >= semver::Version::new(0, 3, 10) {
            command.arg("--no-optimize");
        }
        command.arg(path);

        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;

        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let stdout = String::from_utf8_lossy(output.stdout.as_slice()).to_string();

        Ok(stdout)
    }

    ///
    /// Returns all the Vyper data required to compile the contracts at `paths`.
    ///
    pub fn batch(
        &self,
        version: &semver::Version,
        mut paths: Vec<PathBuf>,
        evm_version: Option<compiler_llvm_context::EVMVersion>,
        optimize: bool,
    ) -> anyhow::Result<Project> {
        paths.sort();

        let mut command = std::process::Command::new(self.executable.as_str());
        if let Some(evm_version) = evm_version {
            command.arg("--evm-version");
            command.arg(evm_version.to_string());
        }
        command.arg("-f");
        command.arg("ir_json,metadata,method_identifiers,ast");
        if !optimize || self.version.default >= semver::Version::new(0, 3, 10) {
            command.arg("--no-optimize");
        }
        command.args(paths.as_slice());

        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;

        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let stdout = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
        let lines: Vec<&str> = stdout.lines().collect();
        let results: BTreeMap<String, anyhow::Result<VyperContract>> = paths
            .into_par_iter()
            .zip(lines.into_par_iter().chunks(VyperContract::EXPECTED_LINES))
            .map(|(path, group)| {
                let path_str = path.to_string_lossy().to_string();
                let source_code = match std::fs::read_to_string(path).map_err(|error| {
                    anyhow::anyhow!("Source code file `{}` reading error: {}", path_str, error)
                }) {
                    Ok(source_code) => source_code,
                    Err(error) => return (path_str, Err(error)),
                };

                if let Err(error) = Self::check_unsupported(source_code.as_str()) {
                    let error = anyhow::anyhow!("Contract `{}`: {}", path_str, error);
                    return (path_str, Err(error));
                }

                let contract_result =
                    VyperContract::try_from_lines(version.to_owned(), source_code, group.to_vec())
                        .map_err(|error| {
                            anyhow::anyhow!(
                                "Contract `{}` JSON output parsing: {}",
                                path_str,
                                error
                            )
                        });

                (path_str, contract_result)
            })
            .collect();
        let contracts =
            results
                .into_iter()
                .try_fold(BTreeMap::new(), |mut accumulator, (path, result)| {
                    accumulator.insert(path, result?.into());
                    Ok::<BTreeMap<String, Contract>, anyhow::Error>(accumulator)
                })?;

        let mut source_code_hasher = sha3::Keccak256::new();
        for (_path, contract) in contracts.iter() {
            source_code_hasher.update(contract.source_code().as_bytes());
        }
        let source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD] =
            source_code_hasher.finalize_fixed().into();

        let project = Project::new(version.to_owned(), source_code_hash, contracts);

        Ok(project)
    }

    ///
    /// The `vyper -f <identifiers> ...` mirror.
    ///
    pub fn extra_output(
        &self,
        path: &Path,
        evm_version: Option<compiler_llvm_context::EVMVersion>,
        extra_output: &str,
    ) -> anyhow::Result<String> {
        let mut command = std::process::Command::new(self.executable.as_str());
        if let Some(evm_version) = evm_version {
            command.arg("--evm-version");
            command.arg(evm_version.to_string());
        }
        command.arg("-f");
        command.arg(extra_output);
        command.arg(path);
        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        Ok(String::from_utf8_lossy(output.stdout.as_slice()).to_string())
    }

    ///
    /// Checks for unsupported code is a Vyper source code file.
    ///
    pub fn check_unsupported(source_code: &str) -> anyhow::Result<()> {
        for function in [
            crate::r#const::FORBIDDEN_FUNCTION_NAME_CREATE_COPY_OF,
            crate::r#const::FORBIDDEN_FUNCTION_NAME_CREATE_FROM_BLUEPRINT,
        ] {
            if source_code.contains(function) {
                return Err(anyhow::anyhow!(
                    "Built-in function `{}` is not supported",
                    function
                ));
            }
        }

        Ok(())
    }

    ///
    /// The `vyper --version` mini-parser.
    ///
    fn version(executable: &str) -> anyhow::Result<Version> {
        let mut command = std::process::Command::new(executable);
        command.arg("--version");
        let output = command
            .output()
            .map_err(|error| anyhow::anyhow!("{} subprocess error: {:?}", executable, error))?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let stdout = String::from_utf8_lossy(output.stdout.as_slice());
        let long = stdout.to_string();
        let default: semver::Version = long
            .split('+')
            .next()
            .ok_or_else(|| anyhow::anyhow!("{} version parsing: metadata dropping", executable))?
            .parse()
            .map_err(|error| anyhow::anyhow!("{} version parsing: {}", executable, error))?;

        let version = Version::new(long, default);
        if !Self::SUPPORTED_VERSIONS.contains(&version.default) {
            anyhow::bail!(
                "Only `vyper` versions [ {} ] are supported, found {}",
                Self::SUPPORTED_VERSIONS
                    .into_iter()
                    .map(|version| version.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                version.default,
            );
        }

        Ok(version)
    }
}
