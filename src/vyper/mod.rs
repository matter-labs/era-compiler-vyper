//!
//! The Vyper compiler.
//!

pub mod combined_json;
pub mod selection;
pub mod standard_json;
pub mod version;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::RwLock;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::project::contract::vyper::Contract as VyperContract;
use crate::project::contract::Contract;
use crate::project::Project;

use self::selection::Selection;
use self::standard_json::input::settings::optimize::Optimize as StandardJsonInputSettingsOptimize;
use self::standard_json::input::Input as StandardJsonInput;
use self::standard_json::output::Output as StandardJsonOutput;
use self::version::Version;

///
/// The Vyper compiler.
///
#[derive(Debug, Clone)]
pub struct Compiler {
    /// The binary executable name.
    pub executable: String,
    /// The `vyper` compiler version.
    pub version: Version,
}

impl Compiler {
    /// The default executable name.
    pub const DEFAULT_EXECUTABLE_NAME: &'static str = "vyper";

    /// The supported versions of `vyper`.
    pub const SUPPORTED_VERSIONS: [semver::Version; 4] = [
        semver::Version::new(0, 3, 3),
        semver::Version::new(0, 3, 9),
        semver::Version::new(0, 3, 10),
        semver::Version::new(0, 4, 0),
    ];

    /// The first version where we cannot use the optimizer.
    pub const FIRST_VERSION_OPTIMIZER_UNUSABLE: semver::Version = semver::Version::new(0, 3, 10);

    /// The first version supporting `--enable-decimals`.
    pub const FIRST_VERSION_ENABLE_DECIMALS_SUPPORT: semver::Version =
        semver::Version::new(0, 4, 0);

    ///
    /// A shortcut constructor.
    ///
    /// Different tools may use different `executable` names. For example, the integration tester
    /// uses `vyper-<version>` format.
    ///
    pub fn new(executable: &str) -> anyhow::Result<Self> {
        if let Some(executable) = Self::executables()
            .read()
            .expect("Sync")
            .get(executable)
            .cloned()
        {
            return Ok(executable);
        }
        let mut executables = Self::executables().write().expect("Sync");

        if let Err(error) = which::which(executable) {
            anyhow::bail!(
                "The `{executable}` executable not found in ${{PATH}}: {}",
                error
            );
        }
        let version = Self::parse_version(executable)?;
        let compiler = Self {
            executable: executable.to_owned(),
            version,
        };

        executables.insert(executable.to_owned(), compiler.clone());
        Ok(compiler)
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
        command.stderr(std::process::Stdio::piped());
        command.arg("--standard-json");

        if self.version.default >= Self::FIRST_VERSION_OPTIMIZER_UNUSABLE {
            input.settings.optimize = StandardJsonInputSettingsOptimize::None;
        }

        let mut process = command.spawn().map_err(|error| {
            anyhow::anyhow!("{} subprocess spawning error: {error:?}", self.executable)
        })?;
        let stdin = process.stdin.take().ok_or_else(|| {
            anyhow::anyhow!("{:?} subprocess stdin getting error", self.executable)
        })?;
        serde_json::to_writer(stdin, &input).map_err(|error| {
            anyhow::anyhow!(
                "{} subprocess stdin writing error: {error:?}",
                self.executable
            )
        })?;

        let result = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!(
                "{} subprocess output reading error: {error:?}",
                self.executable
            )
        })?;
        let mut output = match era_compiler_common::deserialize_from_slice::<StandardJsonOutput>(
            result.stdout.as_slice(),
        ) {
            Ok(output) => output,
            Err(error) => {
                anyhow::bail!(
                    "{} subprocess stdout parsing error: {error:?} (stderr: {})",
                    self.executable,
                    String::from_utf8_lossy(result.stderr.as_slice())
                );
            }
        };
        if !result.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(result.stderr.as_slice())
            );
        }

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
    /// Returns all the Vyper data required to compile the contracts at `paths`.
    ///
    pub fn batch(
        &self,
        version: &semver::Version,
        mut paths: Vec<PathBuf>,
        mut selection: Vec<Selection>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        enable_decimals: bool,
        optimize: bool,
    ) -> anyhow::Result<Project> {
        paths.sort();

        let output_selection: Vec<Selection> = selection.clone();

        let extra_selection = [
            Selection::IR,
            Selection::IRJson,
            Selection::Metadata,
            Selection::AST,
            Selection::ABI,
            Selection::MethodIdentifiers,
        ];
        selection.extend(
            extra_selection
                .iter()
                .filter(|flag| !selection.contains(flag))
                .collect::<Vec<&Selection>>(),
        );

        let mut command = std::process::Command::new(self.executable.as_str());
        if let Some(evm_version) = evm_version {
            command.arg("--evm-version");
            command.arg(evm_version.to_string());
        }
        if enable_decimals && self.version.default >= Self::FIRST_VERSION_ENABLE_DECIMALS_SUPPORT {
            command.arg("--enable-decimals");
        }
        command.arg("-f");
        command.arg(
            selection
                .iter()
                .filter_map(|selection| {
                    if selection.is_supported_by_vyper() {
                        Some(selection.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(","),
        );
        if self.version.default >= Self::FIRST_VERSION_OPTIMIZER_UNUSABLE {
            command.arg("--optimize");
            command.arg("none");
        } else if !optimize {
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
            .zip(lines.into_par_iter().chunks(selection.len()))
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

                let contract_result = VyperContract::try_from_lines(
                    version.to_owned(),
                    source_code,
                    &selection,
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

        let project = Project::new(version.to_owned(), contracts, output_selection);

        Ok(project)
    }

    ///
    /// Checks for unsupported code is a Vyper source code file.
    ///
    pub fn check_unsupported(source_code: &str) -> anyhow::Result<()> {
        for function in [crate::r#const::FORBIDDEN_FUNCTION_NAME_CREATE_COPY_OF] {
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
    /// Returns the global shared array of `vyper` executables.
    ///
    fn executables() -> &'static RwLock<HashMap<String, Self>> {
        static EXECUTABLES: OnceLock<RwLock<HashMap<String, Compiler>>> = OnceLock::new();
        EXECUTABLES.get_or_init(|| RwLock::new(HashMap::new()))
    }

    ///
    /// The `vyper --version` mini-parser.
    ///
    fn parse_version(executable: &str) -> anyhow::Result<Version> {
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
