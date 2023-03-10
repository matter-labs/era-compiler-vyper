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
use std::sync::Arc;
use std::sync::Mutex;

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
}

impl Compiler {
    /// The default executable name.
    pub const DEFAULT_EXECUTABLE_NAME: &'static str = "vyper";

    /// The supported version of `vyper`.
    pub const SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 3, 3);

    ///
    /// A shortcut constructor.
    ///
    /// Different tools may use different `executable` names. For example, the integration tester
    /// uses `vyper-<version>` format.
    ///
    pub fn new(executable: String) -> Self {
        Self { executable }
    }

    ///
    /// The `vyper -f combined_json input_files...` mirror.
    ///
    pub fn combined_json(&self, paths: &[PathBuf]) -> anyhow::Result<CombinedJson> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.arg("-f");
        command.arg("combined_json");
        command.args(paths);
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

        let combined_json = serde_json::from_slice(output.stdout.as_slice()).expect("Always valid");

        Ok(combined_json)
    }

    ///
    /// The `vyper --standard-json` mirror.
    ///
    pub fn standard_json(&self, input: StandardJsonInput) -> anyhow::Result<StandardJsonOutput> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.arg("--standard-json");

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

        let mut output: StandardJsonOutput = serde_json::from_slice(output.stdout.as_slice())
            .map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess output parsing error: {}\n{}",
                    self.executable,
                    error,
                    serde_json::from_slice::<serde_json::Value>(output.stdout.as_slice())
                        .map(|json| serde_json::to_string_pretty(&json).expect("Always valid"))
                        .unwrap_or_else(
                            |_| String::from_utf8_lossy(output.stdout.as_slice()).to_string()
                        ),
                )
            })?;
        for (full_path, source) in input.sources.into_iter() {
            let mut path_split = full_path.split(':');
            let file_path = path_split.next().ok_or_else(|| {
                anyhow::anyhow!("Cannot get the file path from full path `{}`", full_path)
            })?;
            let contract_name = path_split.next().ok_or_else(|| {
                anyhow::anyhow!(
                    "Cannot get the contract name from full path `{}`",
                    full_path
                )
            })?;
            output
                .files
                .as_mut()
                .ok_or_else(|| anyhow::anyhow!("No contracts in the standard JSON output"))?
                .get_mut(file_path)
                .ok_or_else(|| {
                    anyhow::anyhow!("File `{}` not found in the standard JSON output", file_path)
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
    pub fn lll_debug(&self, path: &Path, optimize: bool) -> anyhow::Result<String> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.arg("-f");
        command.arg("ir");
        if !optimize {
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
        optimize: bool,
    ) -> anyhow::Result<Project> {
        paths.sort();

        let mut command = std::process::Command::new(self.executable.as_str());
        command.arg("-f");
        command.arg("ir_json,metadata,method_identifiers");
        if !optimize {
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
        let source_code_hasher = Arc::new(Mutex::new(sha3::Keccak256::new()));
        let results: BTreeMap<String, anyhow::Result<VyperContract>> = paths
            .into_par_iter()
            .zip(lines.into_par_iter().chunks(3))
            .map(|(path, group)| {
                let path_str = path.to_string_lossy().to_string();
                match std::fs::read_to_string(path).map_err(|error| {
                    anyhow::anyhow!("Source code file `{}` reading error: {}", path_str, error)
                }) {
                    Ok(source_code) => source_code_hasher
                        .lock()
                        .expect("Sync")
                        .update(source_code.as_bytes()),
                    Err(error) => return (path_str, Err(error)),
                }

                let contract_result = VyperContract::try_from_lines(
                    version.to_owned(),
                    group.to_vec(),
                )
                .map_err(|error| {
                    anyhow::anyhow!("Contract `{}` JSON output parsing: {}", path_str, error)
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
        let source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD] =
            Arc::try_unwrap(source_code_hasher)
                .expect("Sync")
                .into_inner()
                .expect("Sync")
                .finalize_fixed()
                .into();

        let project = Project::new(version.to_owned(), source_code_hash, contracts);

        Ok(project)
    }

    ///
    /// The `vyper -f <identifiers> ...` mirror.
    ///
    pub fn extra_output(&self, path: &Path, extra_output: &str) -> anyhow::Result<String> {
        let mut command = std::process::Command::new(self.executable.as_str());
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
    /// The `vyper --version` mini-parser.
    ///
    pub fn version(&self) -> anyhow::Result<Version> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.arg("--version");
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

        let stdout = String::from_utf8_lossy(output.stdout.as_slice());
        let long = stdout.to_string();
        let default: semver::Version = long
            .split('+')
            .next()
            .ok_or_else(|| {
                anyhow::anyhow!("{} version parsing: metadata dropping", self.executable)
            })?
            .parse()
            .map_err(|error| anyhow::anyhow!("{} version parsing: {}", self.executable, error))?;

        let version = Version::new(long, default);
        if version.default != Self::SUPPORTED_VERSION {
            anyhow::bail!(
                "`vyper` versions !={} are not supported, found {}",
                Self::SUPPORTED_VERSION,
                version.default,
            );
        }

        Ok(version)
    }
}
