//!
//! Vyper compiler library.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::assigning_clones)]

pub(crate) mod build;
pub(crate) mod r#const;
pub(crate) mod message_type;
pub(crate) mod metadata;
pub(crate) mod process;
pub(crate) mod project;
pub(crate) mod vyper;

pub use self::build::contract::Contract as ContractBuild;
pub use self::build::Build;
pub use self::message_type::MessageType;
pub use self::metadata::function::Function as FunctionMetadata;
pub use self::metadata::Metadata;
pub use self::process::input::Input as ProcessInput;
pub use self::process::output::Output as ProcessOutput;
pub use self::process::run as run_recursive;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract;
pub use self::project::Project;
pub use self::r#const::*;
pub use self::vyper::combined_json::contract::Contract as VyperCompilerCombinedJsonContract;
pub use self::vyper::combined_json::CombinedJson as VyperCompilerCombinedJson;
pub use self::vyper::selection::Selection as VyperSelection;
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

mod tests;

use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use path_slash::PathExt;

///
/// Runs the LLVM IR mode.
///
pub fn llvm_ir(
    input_paths: Vec<PathBuf>,
    output_selection: &[VyperSelection],
    include_metadata_hash: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_messages: Vec<MessageType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    if input_paths.is_empty() {
        writeln!(std::io::stderr(), "No input sources provided").expect("Stderr writing error");
    }

    let paths: Vec<&Path> = input_paths.iter().map(|path| path.as_path()).collect();
    let project = Project::try_from_llvm_ir_paths(paths.as_slice(), output_selection)?;

    let build = project.compile(
        None,
        include_metadata_hash,
        optimizer_settings,
        llvm_options,
        suppressed_messages,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the EraVM assembly mode.
///
pub fn eravm_assembly(
    input_paths: Vec<PathBuf>,
    output_selection: &[VyperSelection],
    include_metadata_hash: bool,
    llvm_options: Vec<String>,
    suppressed_messages: Vec<MessageType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    if input_paths.is_empty() {
        writeln!(std::io::stderr(), "No input sources provided").expect("Stderr writing error");
    }

    let paths: Vec<&Path> = input_paths.iter().map(|path| path.as_path()).collect();
    let project = Project::try_from_eravm_assembly_paths(paths.as_slice(), output_selection)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::cycles();
    let build = project.compile(
        None,
        include_metadata_hash,
        optimizer_settings,
        llvm_options,
        suppressed_messages,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the standard output mode.
///
pub fn standard_output(
    input_paths: Vec<PathBuf>,
    vyper: &VyperCompiler,
    output_selection: &[VyperSelection],
    evm_version: Option<era_compiler_common::EVMVersion>,
    enable_decimals: bool,
    include_metadata_hash: bool,
    vyper_optimizer_enabled: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_messages: Vec<MessageType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let project = vyper.batch(
        &vyper.version.default,
        input_paths,
        output_selection,
        evm_version,
        enable_decimals,
        vyper_optimizer_enabled,
    )?;

    if let Some(ref debug_config) = debug_config {
        for (path, contract) in project.contracts.iter() {
            if let Some(ir_string) = contract.ir_string() {
                debug_config.dump_lll(path.as_str(), None, ir_string.as_str())?;
            }
        }
    }

    let build = project.compile(
        evm_version,
        include_metadata_hash,
        optimizer_settings,
        llvm_options,
        suppressed_messages,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the combined JSON mode.
///
pub fn combined_json(
    input_paths: Vec<PathBuf>,
    vyper: &VyperCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    enable_decimals: bool,
    include_metadata_hash: bool,
    vyper_optimizer_enabled: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_messages: Vec<MessageType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<VyperCompilerCombinedJson> {
    let zkvyper_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let output_selection = vec![
        VyperSelection::ABI,
        VyperSelection::MethodIdentifiers,
        VyperSelection::StorageLayout,
        VyperSelection::UserDocumentation,
        VyperSelection::DeveloperDocumentation,
    ];

    let project: Project = vyper.batch(
        &vyper.version.default,
        input_paths,
        output_selection.as_slice(),
        evm_version,
        enable_decimals,
        vyper_optimizer_enabled,
    )?;

    if let Some(ref debug_config) = debug_config {
        for (path, contract) in project.contracts.iter() {
            if let Some(ir_string) = contract.ir_string() {
                debug_config.dump_lll(path.as_str(), None, ir_string.as_str())?;
            }
        }
    }

    let build = project.compile(
        evm_version,
        include_metadata_hash,
        optimizer_settings,
        llvm_options,
        suppressed_messages,
        debug_config,
    )?;

    let combined_json = build.into_combined_json(Some(&vyper.version.default), &zkvyper_version);

    Ok(combined_json)
}

///
/// Normalizes an input path by converting it to POSIX format.
///
pub fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
    let path = path
        .to_slash()
        .ok_or_else(|| anyhow::anyhow!("Error: input path {path:?} POSIX conversion error"))?
        .to_string();
    let path = PathBuf::from(path.as_str());
    Ok(path)
}
