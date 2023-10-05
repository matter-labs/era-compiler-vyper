//!
//! Vyper to EraVM compiler binary.
//!

pub mod arguments;

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
fn main() {
    std::process::exit(match main_inner() {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let mut arguments = Arguments::new();
    arguments.validate()?;

    rayon::ThreadPoolBuilder::new()
        .stack_size(RAYON_WORKER_STACK_SIZE)
        .build_global()
        .expect("Thread pool configuration failure");
    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target(compiler_llvm_context::Target::EraVM); // TODO: pass from CLI

    if arguments.version {
        println!(
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            inkwell::support::get_commit_id().to_string(),
        );
        return Ok(());
    }

    if arguments.recursive_process {
        return compiler_vyper::run_process();
    }

    let debug_config = match arguments.debug_output_directory {
        Some(debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            Some(compiler_llvm_context::DebugConfig::new(
                debug_output_directory,
            ))
        }
        None => None,
    };

    for path in arguments.input_files.iter_mut() {
        *path = path.canonicalize()?;
    }

    let suppressed_warnings = match arguments.suppress_warnings {
        Some(warnings) => compiler_vyper::WarningType::try_from_strings(warnings.as_slice())?,
        None => vec![],
    };

    let vyper = compiler_vyper::VyperCompiler::new(
        arguments
            .vyper
            .unwrap_or_else(|| compiler_vyper::VyperCompiler::DEFAULT_EXECUTABLE_NAME.to_owned()),
    );

    let mut optimizer_settings = match arguments.optimization {
        Some(mode) => compiler_llvm_context::OptimizerSettings::try_from_cli(mode)?,
        None => compiler_llvm_context::OptimizerSettings::cycles(),
    };
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

    let include_metadata_hash = match arguments.metadata_hash {
        Some(metadata_hash) => {
            let metadata =
                compiler_llvm_context::EraVMMetadataHash::from_str(metadata_hash.as_str())?;
            metadata != compiler_llvm_context::EraVMMetadataHash::None
        }
        None => true,
    };

    let build = if arguments.llvm_ir {
        compiler_vyper::llvm_ir(
            arguments.input_files,
            optimizer_settings,
            include_metadata_hash,
            suppressed_warnings,
            debug_config,
        )
    } else if arguments.zkasm {
        compiler_vyper::zkasm(
            arguments.input_files,
            include_metadata_hash,
            suppressed_warnings,
            debug_config,
        )
    } else {
        match arguments.format.as_deref() {
            Some("combined_json") => {
                compiler_vyper::combined_json(
                    arguments.input_files,
                    &vyper,
                    !arguments.disable_vyper_optimizer,
                    optimizer_settings,
                    include_metadata_hash,
                    suppressed_warnings,
                    debug_config,
                    arguments.output_directory,
                    arguments.overwrite,
                )?;
                return Ok(());
            }
            Some(format) if format.split(',').any(|format| format == "combined_json") => {
                anyhow::bail!("If using combined_json it must be the only output format requested");
            }
            Some(_) | None => compiler_vyper::standard_output(
                arguments.input_files,
                &vyper,
                !arguments.disable_vyper_optimizer,
                optimizer_settings,
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
                    eprintln!("\n{}", warning);
                }
            }

            std::fs::create_dir_all(output_directory.as_path())?;

            build.write_to_directory(output_directory.as_path(), arguments.overwrite)?;
        }
        None => {
            for (path, contract) in build.contracts.into_iter() {
                eprintln!("Contract `{path}`:");
                for warning in contract.warnings.iter() {
                    eprintln!("\n{}", warning);
                }

                let bytecode_string = hex::encode(contract.build.bytecode);
                println!("0x{bytecode_string}");

                if let Some(format) = arguments.format.as_deref() {
                    let extra_output = vyper.extra_output(PathBuf::from(path).as_path(), format)?;
                    println!();
                    println!("{extra_output}");
                }
            }
        }
    }

    Ok(())
}
