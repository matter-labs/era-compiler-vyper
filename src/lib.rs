//!
//! Vyper to EraVM compiler library.
//!

pub(crate) mod build;
pub(crate) mod r#const;
pub(crate) mod metadata;
pub(crate) mod process;
pub(crate) mod project;
pub(crate) mod vyper;
pub(crate) mod warning_type;

pub use self::build::contract::Contract as ContractBuild;
pub use self::build::Build;
pub use self::metadata::function::Function as FunctionMetadata;
pub use self::metadata::Metadata;
pub use self::process::input::Input as ProcessInput;
pub use self::process::output::Output as ProcessOutput;
pub use self::process::run as run_process;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract;
pub use self::project::Project;
pub use self::r#const::*;
pub use self::vyper::combined_json::contract::Contract as VyperCompilerCombinedJsonContract;
pub use self::vyper::combined_json::CombinedJson as VyperCompilerCombinedJson;
pub use self::vyper::standard_json::input::language::Language as VyperCompilerStandardInputJsonLanguage;
pub use self::vyper::standard_json::input::settings::selection::Selection as VyperCompilerStandardInputJsonSettingsSelection;
pub use self::vyper::standard_json::input::settings::Settings as VyperCompilerStandardInputJsonSettings;
pub use self::vyper::standard_json::input::source::Source as VyperCompilerStandardInputJsonSource;
pub use self::vyper::standard_json::input::Input as VyperCompilerStandardInputJson;
pub use self::vyper::standard_json::output::contract::evm::EVM as VyperCompilerStandardOutputJsonContractEVMObject;
pub use self::vyper::standard_json::output::contract::Contract as VyperCompilerStandardOutputJsonContract;
pub use self::vyper::standard_json::output::error::Error as VyperCompilerStandardOutputJsonError;
pub use self::vyper::standard_json::output::Output as VyperCompilerStandardOutputJson;
pub use self::vyper::version::Version as VyperVersion;
pub use self::vyper::Compiler as VyperCompiler;
pub use self::warning_type::WarningType;

mod tests;

use std::io::Write;
use std::path::PathBuf;

///
/// Runs the LLVM IR mode.
///
pub fn llvm_ir(
    mut input_files: Vec<PathBuf>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    include_metadata_hash: bool,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let path = match input_files.len() {
        1 => input_files.remove(0),
        0 => anyhow::bail!("The input file is missing"),
        length => anyhow::bail!(
            "Only one input file is allowed in LLVM IR mode, but found {}",
            length
        ),
    };

    let project = Project::try_from_llvm_ir_path(&path)?;

    let build = project.compile(
        None,
        optimizer_settings,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        suppressed_warnings,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the EraVM assembly mode.
///
pub fn zkasm(
    mut input_files: Vec<PathBuf>,
    include_metadata_hash: bool,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let path = match input_files.len() {
        1 => input_files.remove(0),
        0 => anyhow::bail!("The input file is missing"),
        length => anyhow::bail!(
            "Only one input file is allowed in EraVM assembly mode, but found {}",
            length
        ),
    };

    let project = Project::try_from_zkasm_path(&path)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();
    let build = project.compile(
        None,
        optimizer_settings,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        suppressed_warnings,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the standard output mode.
///
#[allow(clippy::too_many_arguments)]
pub fn standard_output(
    input_files: Vec<PathBuf>,
    vyper: &VyperCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    vyper_optimizer_enabled: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    include_metadata_hash: bool,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    if let Some(ref debug_config) = debug_config {
        for path in input_files.iter() {
            let lll_debug = vyper.lll_debug(path.as_path(), evm_version, true)?;
            debug_config.dump_lll(path.to_string_lossy().as_ref(), None, lll_debug.as_str())?;
        }
    }

    let project = vyper.batch(
        &vyper.version.default,
        input_files,
        evm_version,
        vyper_optimizer_enabled,
    )?;

    let build = project.compile(
        evm_version,
        optimizer_settings,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        suppressed_warnings,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the combined JSON mode.
///
#[allow(clippy::too_many_arguments)]
pub fn combined_json(
    input_files: Vec<PathBuf>,
    vyper: &VyperCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    vyper_optimizer_enabled: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    include_metadata_hash: bool,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
) -> anyhow::Result<()> {
    let zkvyper_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    if let Some(ref debug_config) = debug_config {
        for path in input_files.iter() {
            let lll_debug =
                vyper.lll_debug(path.as_path(), evm_version, vyper_optimizer_enabled)?;
            debug_config.dump_lll(path.to_string_lossy().as_ref(), None, lll_debug.as_str())?;
        }
    }

    let project: Project = vyper.batch(
        &vyper.version.default,
        input_files.clone(),
        evm_version,
        vyper_optimizer_enabled,
    )?;

    let build = project.compile(
        evm_version,
        optimizer_settings,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        suppressed_warnings,
        debug_config,
    )?;

    let mut combined_json =
        vyper.combined_json(input_files.as_slice(), evm_version, vyper_optimizer_enabled)?;
    build.write_to_combined_json(&mut combined_json, &zkvyper_version)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;

            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;
        }
        None => writeln!(
            std::io::stdout(),
            "{}",
            serde_json::to_string(&combined_json).expect("Always valid")
        )?,
    }
    std::process::exit(0);
}
