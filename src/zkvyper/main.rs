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
    if let Some(llvm_options) = arguments.llvm_options {
        let llvm_options = shell_words::split(llvm_options.as_str())
            .map_err(|error| anyhow::anyhow!("LLVM options parsing error: {}", error))?;
        let llvm_options = Vec::from_iter(llvm_options.iter().map(String::as_str));
        inkwell::support::parse_command_line_options(
            llvm_options.len() as i32,
            llvm_options.as_slice(),
            "",
        );
    }

    let vyper = compiler_vyper::VyperCompiler::new(
        arguments
            .vyper
            .unwrap_or_else(|| compiler_vyper::VyperCompiler::DEFAULT_EXECUTABLE_NAME.to_owned()),
    );

    let build = if arguments.llvm_ir {
        compiler_vyper::llvm_ir(arguments.input_files, !arguments.no_optimize, debug_config)
    } else {
        match arguments.format.as_deref() {
            Some("combined_json") => {
                compiler_vyper::combined_json(
                    arguments.input_files,
                    &vyper,
                    !arguments.no_optimize,
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
                !arguments.no_optimize,
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
