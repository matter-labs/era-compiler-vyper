//!
//! Process for compiling a single compilation unit.
//!

pub mod input;
pub mod output;

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

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
    if input.enable_test_encoding {
        zkevm_assembly::set_encoding_mode(zkevm_assembly::RunningVmEncodingMode::Testing);
    }

    let build = input.contract.into_owned().compile(
        input.full_path.as_str(),
        input.source_code_hash,
        input.evm_version,
        input.optimizer_settings,
        input.llvm_options,
        input.suppressed_warnings,
        input.debug_config,
    )?;

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
pub fn call<I, O>(input: I) -> anyhow::Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    let executable = match EXECUTABLE.get() {
        Some(executable) => executable.to_owned(),
        None => std::env::current_exe()?,
    };

    let mut command = Command::new(executable.as_path());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.arg("--recursive-process");

    let mut process = command
        .spawn()
        .map_err(|error| anyhow::anyhow!("{executable:?} subprocess spawning error: {error:?}"))?;

    let stdin = process
        .stdin
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("{executable:?} subprocess stdin getting error"))?;
    let stdin_input = serde_json::to_vec(&input).expect("Always valid");
    stdin.write_all(stdin_input.as_slice()).map_err(|error| {
        anyhow::anyhow!("{executable:?} subprocess stdin writing error: {error:?}",)
    })?;

    let result = process.wait_with_output().map_err(|error| {
        anyhow::anyhow!("{executable:?} subprocess output reading error: {error:?}")
    })?;
    let stderr_message = String::from_utf8_lossy(result.stderr.as_slice());
    if !result.status.success() {
        anyhow::bail!("{}", stderr_message.trim());
    }
    let output = match era_compiler_common::deserialize_from_slice::<O>(result.stdout.as_slice()) {
        Ok(output) => output,
        Err(error) => {
            anyhow::bail!("{executable:?} subprocess stdout parsing error: {error:?} (stderr: {stderr_message})");
        }
    };

    Ok(output)
}
