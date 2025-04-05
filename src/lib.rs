//!
//! Vyper compiler library.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::assigning_clones)]

pub mod build;
pub mod r#const;
pub mod process;
pub mod project;
pub mod vyper;
pub mod warning_type;

pub use self::build::contract::Contract as ContractBuild;
pub use self::build::Build;
pub use self::process::input::Input as ProcessInput;
pub use self::process::output::Output as ProcessOutput;
pub use self::process::run as run_recursive;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract;
pub use self::project::Project;
pub use self::r#const::*;
pub use self::vyper::combined_json::contract::Contract as VyperCompilerCombinedJsonContract;
pub use self::vyper::combined_json::CombinedJson as VyperCompilerCombinedJson;
pub use self::vyper::selector::Selector as VyperSelector;
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

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use path_slash::PathExt;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

///
/// Runs the LLVM IR mode.
///
pub fn llvm_ir(
    input_paths: Vec<PathBuf>,
    output_selection: &[VyperSelector],
    metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
    append_bytecode_metadata: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let paths: Vec<&Path> = input_paths.iter().map(|path| path.as_path()).collect();
    let project = Project::try_from_llvm_ir_paths(paths.as_slice(), output_selection)?;

    let mut build = project.compile(
        None,
        metadata_hash_type,
        append_bytecode_metadata,
        optimizer_settings,
        llvm_options,
        suppressed_warnings,
        debug_config,
    )?;
    build.link(BTreeMap::new())?;
    Ok(build)
}

///
/// Runs the EraVM assembly mode.
///
pub fn eravm_assembly(
    input_paths: Vec<PathBuf>,
    output_selection: &[VyperSelector],
    metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
    append_bytecode_metadata: bool,
    llvm_options: Vec<String>,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let paths: Vec<&Path> = input_paths.iter().map(|path| path.as_path()).collect();
    let project = Project::try_from_eravm_assembly_paths(paths.as_slice(), output_selection)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::cycles();
    let mut build = project.compile(
        None,
        metadata_hash_type,
        append_bytecode_metadata,
        optimizer_settings,
        llvm_options,
        suppressed_warnings,
        debug_config,
    )?;
    build.link(BTreeMap::new())?;
    Ok(build)
}

///
/// Runs the standard output mode.
///
pub fn standard_output(
    input_paths: Vec<PathBuf>,
    vyper: &VyperCompiler,
    output_selection: &[VyperSelector],
    evm_version: Option<era_compiler_common::EVMVersion>,
    enable_decimals: bool,
    search_paths: Option<Vec<String>>,
    metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
    append_bytecode_metadata: bool,
    vyper_optimizer_enabled: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let project = vyper.batch(
        &vyper.version.default,
        input_paths,
        output_selection,
        evm_version,
        enable_decimals,
        search_paths,
        vyper_optimizer_enabled,
    )?;

    if let Some(ref debug_config) = debug_config {
        for (path, contract) in project.contracts.iter() {
            debug_config.dump_lll(
                path.as_str(),
                contract.ir_string().unwrap_or_default().as_str(),
            )?;
        }
    }

    let mut build = project.compile(
        evm_version,
        metadata_hash_type,
        append_bytecode_metadata,
        optimizer_settings,
        llvm_options,
        suppressed_warnings,
        debug_config,
    )?;
    build.link(BTreeMap::new())?;
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
    search_paths: Option<Vec<String>>,
    metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
    append_bytecode_metadata: bool,
    vyper_optimizer_enabled: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_warnings: Vec<WarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<VyperCompilerCombinedJson> {
    let zkvyper_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let output_selection: Vec<VyperSelector> = vec![
        VyperSelector::IRJson,
        VyperSelector::AST,
        VyperSelector::ABI,
        VyperSelector::MethodIdentifiers,
        VyperSelector::Layout,
        VyperSelector::UserDocumentation,
        VyperSelector::DeveloperDocumentation,
        VyperSelector::EraVMAssembly,
        VyperSelector::ProjectMetadata,
    ];

    let project: Project = vyper.batch(
        &vyper.version.default,
        input_paths,
        output_selection.as_slice(),
        evm_version,
        enable_decimals,
        search_paths,
        vyper_optimizer_enabled,
    )?;

    if let Some(ref debug_config) = debug_config {
        for (path, contract) in project.contracts.iter() {
            debug_config.dump_lll(
                path.as_str(),
                contract.ir_string().unwrap_or_default().as_str(),
            )?;
        }
    }

    let mut build = project.compile(
        evm_version,
        metadata_hash_type,
        append_bytecode_metadata,
        optimizer_settings,
        llvm_options,
        suppressed_warnings,
        debug_config,
    )?;
    build.link(BTreeMap::new())?;

    let combined_json = build.into_combined_json(Some(&vyper.version.default), &zkvyper_version);

    Ok(combined_json)
}

///
/// Runs the disassembler for EraVM bytecode file and prints the output to stdout.
///
pub fn disassemble_eravm(paths: Vec<PathBuf>) -> anyhow::Result<()> {
    let bytecodes = paths
        .into_par_iter()
        .map(|path| {
            let hexadecimal_string = std::fs::read_to_string(path.as_path())?;
            let bytecode_hexadecimal = hexadecimal_string
                .trim()
                .strip_prefix("0x")
                .unwrap_or(hexadecimal_string.as_str());
            let bytecode = hex::decode(bytecode_hexadecimal)?;
            Ok((path, bytecode))
        })
        .collect::<anyhow::Result<BTreeMap<PathBuf, Vec<u8>>>>()?;

    let target_machine = era_compiler_llvm_context::TargetMachine::new(
        era_compiler_common::Target::EraVM,
        &era_compiler_llvm_context::OptimizerSettings::cycles(),
        &[],
    )?;

    let disassemblies = bytecodes
        .into_iter()
        .map(|(path, bytecode)| {
            let bytecode_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                bytecode.as_slice(),
                path.to_str().expect("Always valid"),
                false,
            );
            let disassembly =
                era_compiler_llvm_context::eravm_disassemble(&target_machine, &bytecode_buffer)?;
            Ok((path, disassembly))
        })
        .collect::<anyhow::Result<Vec<(PathBuf, String)>>>()?;

    for (path, disassembly) in disassemblies.into_iter() {
        writeln!(std::io::stderr(), "File {path:?} disassembly:\n\n")?;
        writeln!(std::io::stdout(), "{disassembly}")?;
        writeln!(std::io::stderr(), "\n\n")?;
    }
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
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
