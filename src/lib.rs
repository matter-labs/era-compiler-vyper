//!
//! Vyper to zkEVM compiler library.
//!

extern crate core;

pub(crate) mod build;
pub(crate) mod r#const;
pub(crate) mod metadata;
pub(crate) mod project;
pub(crate) mod vyper;

pub use self::build::contract::Contract as ContractBuild;
pub use self::build::Build;
pub use self::metadata::function::Function as FunctionMetadata;
pub use self::metadata::Metadata;
pub use self::project::contract::Contract;
pub use self::project::Project;
pub use self::r#const::*;
pub use self::vyper::combined_json::contract::Contract as VyperCompilerCombinedJsonContract;
pub use self::vyper::combined_json::CombinedJson as VyperCompilerCombinedJson;
pub use self::vyper::standard_json::input::language::Language as VyperCompilerStandardInputJsonLanguage;
pub use self::vyper::standard_json::input::settings::evm_version::EVMVersion as VyperCompilerStandardInputJsonSettingsEVMVersion;
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

use std::path::PathBuf;

///
/// Runs the LLVM IR mode.
///
pub fn llvm_ir(
    mut input_files: Vec<PathBuf>,
    optimize: bool,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let path = match input_files.len() {
        1 => input_files.remove(0),
        0 => anyhow::bail!("The input file is missing"),
        length => anyhow::bail!(
            "Only one input file is allowed in the LLVM IR mode, but found {}",
            length
        ),
    };

    let project = Project::try_from_llvm_ir_path(&path)?;

    let optimizer_settings = if optimize {
        compiler_llvm_context::OptimizerSettings::size()
    } else {
        compiler_llvm_context::OptimizerSettings::none()
    };
    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile(target_machine, optimizer_settings, debug_config)?;

    Ok(build)
}

///
/// Runs the standard output mode.
///
pub fn standard_output(
    input_files: Vec<PathBuf>,
    vyper: &VyperCompiler,
    optimize: bool,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let vyper_version = vyper.version()?;

    if let Some(ref debug_config) = debug_config {
        for path in input_files.iter() {
            let lll_debug = vyper.lll_debug(path.as_path(), true)?;
            debug_config.dump_lll(path.to_string_lossy().as_ref(), lll_debug.as_str())?;
        }
    }

    let project = vyper.batch(&vyper_version.default, input_files, optimize)?;

    let optimizer_settings = if optimize {
        compiler_llvm_context::OptimizerSettings::size()
    } else {
        compiler_llvm_context::OptimizerSettings::none()
    };
    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile(target_machine, optimizer_settings, debug_config)?;

    Ok(build)
}

///
/// Runs the combined JSON mode.
///
pub fn combined_json(
    input_files: Vec<PathBuf>,
    vyper: &VyperCompiler,
    optimize: bool,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
) -> anyhow::Result<()> {
    let vyper_version = vyper.version()?;

    let zkvyper_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    if let Some(ref debug_config) = debug_config {
        for path in input_files.iter() {
            let lll_debug = vyper.lll_debug(path.as_path(), true)?;
            debug_config.dump_lll(path.to_string_lossy().as_ref(), lll_debug.as_str())?;
        }
    }

    let project = vyper.batch(&vyper_version.default, input_files.clone(), optimize)?;

    let optimizer_settings = if optimize {
        compiler_llvm_context::OptimizerSettings::size()
    } else {
        compiler_llvm_context::OptimizerSettings::none()
    };
    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile(target_machine, optimizer_settings, debug_config)?;

    let mut combined_json = vyper.combined_json(input_files.as_slice())?;
    build.write_to_combined_json(&mut combined_json, &zkvyper_version)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;

            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;
        }
        None => println!(
            "{}",
            serde_json::to_string(&combined_json).expect("Always valid")
        ),
    }
    std::process::exit(0);
}
