//!
//! Process for compiling a single compilation unit.
//!

pub mod input;
pub mod output;

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use std::thread::Builder;

use self::input::Input;
use self::output::Output;

/// The overridden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceLock<PathBuf> = OnceLock::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run() -> anyhow::Result<()> {
    let input_json = std::io::read_to_string(std::io::stdin()).expect("Stdin reading error");
    let input: Input = era_compiler_common::deserialize_from_str(input_json.as_str())
        .expect("Stdin reading error");

    let build = Builder::new()
        .stack_size(crate::WORKER_THREAD_STACK_SIZE)
        .spawn(move || {
            input.contract.into_owned().compile(
                input.full_path.as_str(),
                input.metadata_hash,
                input.append_bytecode_metadata,
                input.optimizer_settings,
                input.llvm_options,
                input.output_selection,
                input.suppressed_warnings,
                input.debug_config,
            )
        })
        .expect("Threading error")
        .join()
        .expect("Threading error")?;
    unsafe { inkwell::support::shutdown_llvm() };

    let output = Output::new(build);
    let output_json = serde_json::to_vec(&output).expect("Always valid");
    std::io::stdout()
        .write_all(output_json.as_slice())
        .expect("Stdout writing error");
    Ok(())
}

///
/// Runs this process recursively to compile a single contract.
///
pub fn call<I, O>(path: &str, input: I) -> anyhow::Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    let executable = EXECUTABLE
        .get()
        .cloned()
        .unwrap_or_else(|| std::env::current_exe().expect("Current executable path getting error"));

    let mut command = Command::new(executable.as_path());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.arg("--recursive-process");

    let mut process = command
        .spawn()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess spawning: {error:?}"));

    let stdin = process
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("{executable:?} subprocess stdin getting error"));
    let stdin_input = serde_json::to_vec(&input).expect("Always valid");
    stdin
        .write_all(stdin_input.as_slice())
        .unwrap_or_else(|error| panic!("{executable:?} subprocess stdin writing: {error:?}",));

    let result = process
        .wait_with_output()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess output reading: {error:?}"));

    if result.status.code() != Some(era_compiler_common::EXIT_CODE_SUCCESS) {
        anyhow::bail!(
            "{executable:?} subprocess compiling `{path}` failed with exit code {:?}:\n{}\n{}",
            result.status.code(),
            String::from_utf8_lossy(result.stdout.as_slice()),
            String::from_utf8_lossy(result.stderr.as_slice()),
        );
    }

    let output = match era_compiler_common::deserialize_from_slice::<O>(result.stdout.as_slice()) {
        Ok(output) => output,
        Err(error) => {
            panic!(
                "{executable:?} subprocess stdout parsing error: {error:?}\n{}\n{}",
                String::from_utf8_lossy(result.stdout.as_slice()),
                String::from_utf8_lossy(result.stderr.as_slice()),
            );
        }
    };
    Ok(output)
}
