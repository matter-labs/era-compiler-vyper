//!
//! Vyper to zkEVM compiler binary.
//!

pub mod arguments;

use std::path::PathBuf;

use self::arguments::Arguments;

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

///
/// The application entry point.
///
fn main() {
    std::process::exit(match main_inner() {
        Ok(()) => compiler_common::EXIT_CODE_SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            compiler_common::EXIT_CODE_FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let mut arguments = Arguments::new();

    if arguments.version {
        println!(
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            inkwell::support::get_commit_id().to_string(),
        );
        return Ok(());
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

    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target();

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

    let build = if arguments.llvm_ir {
        compiler_vyper::llvm_ir(arguments.input_files, optimizer_settings, debug_config)
    } else {
        match arguments.format.as_deref() {
            Some("combined_json") => {
                compiler_vyper::combined_json(
                    arguments.input_files,
                    &vyper,
                    optimizer_settings,
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
                optimizer_settings,
                debug_config,
            ),
        }
    }?;

    match arguments.output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;

            build.write_to_directory(output_directory.as_path(), arguments.overwrite)?;
        }
        None => {
            for (path, contract) in build.contracts.into_iter() {
                eprintln!("Contract `{path}`:");
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
