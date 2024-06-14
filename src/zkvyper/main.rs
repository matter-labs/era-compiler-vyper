//!
//! Vyper compiler binary.
//!

pub mod arguments;

use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use self::arguments::Arguments;

/// The rayon worker stack size.
const RAYON_WORKER_STACK_SIZE: usize = 16 * 1024 * 1024;

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

///
/// The application entry point.
///
fn main() -> anyhow::Result<()> {
    std::process::exit(match main_inner() {
        Ok(()) => era_compiler_common::EXIT_CODE_SUCCESS,
        Err(error) => {
            std::io::stderr()
                .write_all(error.to_string().as_bytes())
                .expect("Stderr writing error");
            era_compiler_common::EXIT_CODE_FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let mut arguments = Arguments::new();
    arguments.validate()?;
    arguments.normalize_input_paths()?;

    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(threads) = arguments.threads {
        thread_pool_builder = thread_pool_builder.num_threads(threads);
    }
    thread_pool_builder
        .stack_size(RAYON_WORKER_STACK_SIZE)
        .build_global()
        .expect("Thread pool configuration failure");

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM); // TODO: pass from CLI

    if arguments.version {
        writeln!(
            std::io::stdout(),
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            inkwell::support::get_commit_id().to_string(),
        )?;
        return Ok(());
    }

    if arguments.recursive_process {
        return era_compiler_vyper::run_recursive();
    }

    let debug_config = match arguments.debug_output_directory {
        Some(debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            Some(era_compiler_llvm_context::DebugConfig::new(
                debug_output_directory,
            ))
        }
        None => None,
    };

    let suppressed_warnings = match arguments.suppress_warnings {
        Some(warnings) => era_compiler_vyper::WarningType::try_from_strings(warnings.as_slice())?,
        None => vec![],
    };

    let evm_version = match arguments.evm_version {
        Some(evm_version) => Some(era_compiler_common::EVMVersion::try_from(
            evm_version.as_str(),
        )?),
        None => None,
    };

    let mut optimizer_settings = match arguments.optimization {
        Some(mode) => era_compiler_llvm_context::OptimizerSettings::try_from_cli(mode)?,
        None => era_compiler_llvm_context::OptimizerSettings::cycles(),
    };
    if arguments.fallback_to_optimizing_for_size {
        optimizer_settings.enable_fallback_to_size();
    }
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

    let llvm_options: Vec<String> = arguments
        .llvm_options
        .as_ref()
        .map(|options| options.split(' ').map(|option| option.to_owned()).collect())
        .unwrap_or_default();

    let include_metadata_hash = match arguments.metadata_hash {
        Some(metadata_hash) => {
            let metadata =
                era_compiler_llvm_context::EraVMMetadataHash::from_str(metadata_hash.as_str())?;
            metadata != era_compiler_llvm_context::EraVMMetadataHash::None
        }
        None => true,
    };

    let build = if arguments.llvm_ir {
        era_compiler_vyper::llvm_ir(
            arguments.input_files,
            optimizer_settings,
            llvm_options,
            include_metadata_hash,
            suppressed_warnings,
            debug_config,
        )
    } else if arguments.eravm_assembly {
        era_compiler_vyper::eravm_assembly(
            arguments.input_files,
            llvm_options,
            include_metadata_hash,
            suppressed_warnings,
            debug_config,
        )
    } else {
        let vyper = era_compiler_vyper::VyperCompiler::new(
            arguments
                .vyper
                .as_deref()
                .unwrap_or(era_compiler_vyper::VyperCompiler::DEFAULT_EXECUTABLE_NAME),
        )?;
        match arguments.format.as_deref() {
            Some("combined_json") => {
                era_compiler_vyper::combined_json(
                    arguments.input_files,
                    &vyper,
                    evm_version,
                    !arguments.disable_vyper_optimizer,
                    optimizer_settings,
                    llvm_options,
                    include_metadata_hash,
                    suppressed_warnings,
                    debug_config,
                    arguments.output_directory,
                    arguments.overwrite,
                )?;
                return Ok(());
            }
            Some(format) if format.split(',').any(|format| format == "combined_json") => {
                anyhow::bail!("`combined_json` must be the only output format requested");
            }
            Some(_) | None => era_compiler_vyper::standard_output(
                arguments.input_files,
                &vyper,
                evm_version,
                !arguments.disable_vyper_optimizer,
                optimizer_settings,
                llvm_options,
                include_metadata_hash,
                suppressed_warnings,
                debug_config,
            ),
        }
    }?;

    match arguments.output_directory {
        Some(output_directory) => {
            for (_path, contract) in build.contracts.iter() {
                for warning in contract.warnings.iter() {
                    writeln!(std::io::stderr(), "\n{}", warning)?;
                }
            }

            std::fs::create_dir_all(output_directory.as_path())?;

            build.write_to_directory(output_directory.as_path(), arguments.overwrite)?;
        }
        None => {
            for (path, contract) in build.contracts.into_iter() {
                writeln!(std::io::stderr(), "Contract `{path}`:")?;
                for warning in contract.warnings.iter() {
                    writeln!(std::io::stderr(), "\n{}", warning)?;
                }

                let bytecode_string = hex::encode(contract.build.bytecode);
                writeln!(std::io::stdout(), "0x{bytecode_string}")?;

                if let Some(format) = arguments.format.as_deref() {
                    let vyper = era_compiler_vyper::VyperCompiler::new(
                        arguments
                            .vyper
                            .as_deref()
                            .unwrap_or(era_compiler_vyper::VyperCompiler::DEFAULT_EXECUTABLE_NAME),
                    )?;
                    let extra_output =
                        vyper.extra_output(PathBuf::from(path).as_path(), evm_version, format)?;
                    writeln!(std::io::stdout(), "\n{extra_output}")?;
                }
            }
        }
    }

    Ok(())
}
