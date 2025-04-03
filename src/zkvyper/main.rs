//!
//! Vyper compiler binary.
//!

pub mod arguments;

use std::io::Write;
use std::str::FromStr;

use clap::Parser;

use self::arguments::Arguments;

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
    let mut arguments = Arguments::try_parse()?;
    arguments.validate()?;
    arguments.normalize_input_paths()?;

    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(threads) = arguments.threads {
        thread_pool_builder = thread_pool_builder.num_threads(threads);
    }
    thread_pool_builder
        .stack_size(era_compiler_vyper::WORKER_THREAD_STACK_SIZE)
        .build_global()
        .expect("Thread pool configuration failure");

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM); // TODO: pass from CLI

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

    let debug_config = match arguments.debug_output_dir {
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

    let output_selection = match arguments.format.as_ref() {
        Some(format) => format
            .split(',')
            .map(era_compiler_vyper::VyperSelector::from_str)
            .collect::<anyhow::Result<Vec<era_compiler_vyper::VyperSelector>>>()?,
        None => vec![],
    };
    let is_combined_json =
        output_selection.contains(&era_compiler_vyper::VyperSelector::CombinedJson);
    if is_combined_json && output_selection.len() > 1 {
        anyhow::bail!(
            "`combined_json` cannot be requested together with other output: `{:?}`",
            output_selection,
        );
    }

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
        .map(|options| {
            options
                .split_whitespace()
                .map(|option| option.to_owned())
                .collect()
        })
        .unwrap_or_default();

    let vyper_optimizer_enabled = !arguments.disable_vyper_optimizer;

    let metadata_hash_type = arguments
        .metadata_hash
        .unwrap_or(era_compiler_common::HashType::Keccak256);

    let build = if arguments.llvm_ir {
        era_compiler_vyper::llvm_ir(
            arguments.input_paths,
            output_selection.as_slice(),
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            suppressed_warnings,
            debug_config,
        )
    } else if arguments.eravm_assembly {
        era_compiler_vyper::eravm_assembly(
            arguments.input_paths,
            output_selection.as_slice(),
            metadata_hash_type,
            llvm_options,
            suppressed_warnings,
            debug_config,
        )
    } else if arguments.disassemble {
        return era_compiler_vyper::disassemble_eravm(arguments.input_paths);
    } else {
        let vyper = era_compiler_vyper::VyperCompiler::new(
            arguments
                .vyper
                .as_deref()
                .unwrap_or(era_compiler_vyper::VyperCompiler::DEFAULT_EXECUTABLE_NAME),
        )?;

        if is_combined_json {
            let combined_json = era_compiler_vyper::combined_json(
                arguments.input_paths,
                &vyper,
                arguments.evm_version,
                arguments.enable_decimals,
                arguments.search_paths,
                metadata_hash_type,
                vyper_optimizer_enabled,
                optimizer_settings,
                llvm_options,
                suppressed_warnings,
                debug_config,
            )?;

            match arguments.output_dir {
                Some(output_directory) => {
                    combined_json
                        .write_to_directory(output_directory.as_path(), arguments.overwrite)?;
                }
                None => serde_json::to_writer(std::io::stdout(), &combined_json)
                    .expect("Stdout writing error"),
            }
            std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
        }

        era_compiler_vyper::standard_output(
            arguments.input_paths,
            &vyper,
            output_selection.as_slice(),
            arguments.evm_version,
            arguments.enable_decimals,
            arguments.search_paths,
            metadata_hash_type,
            vyper_optimizer_enabled,
            optimizer_settings,
            llvm_options,
            suppressed_warnings,
            debug_config,
        )
    }?;

    match arguments.output_dir {
        Some(output_directory) => {
            build.write_to_directory(
                output_selection.as_slice(),
                output_directory.as_path(),
                arguments.overwrite,
            )?;
        }
        None => {
            build.write_to_terminal(output_selection.as_slice())?;
        }
    }

    Ok(())
}
