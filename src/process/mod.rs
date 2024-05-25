//!
//! Process for compiling a single compilation unit.
//!

pub mod input;
pub mod output;

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use once_cell::sync::OnceCell;

use self::input::Input;
use self::output::Output;

/// The overridden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceCell<PathBuf> = OnceCell::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run() -> anyhow::Result<()> {
    let input: Input = era_compiler_common::deserialize_from_reader(std::io::stdin())
        .expect("Stdin reading error");
    if input.enable_test_encoding {
        zkevm_assembly::set_encoding_mode(zkevm_assembly::RunningVmEncodingMode::Testing);
    }
    let result = input.contract.into_owned().compile(
        input.full_path.as_str(),
        input.source_code_hash,
        input.evm_version,
        input.optimizer_settings,
        input.suppressed_warnings,
        input.debug_config,
    );

    match result {
        Ok(build) => {
            let output = Output::new(build);
            serde_json::to_writer(std::io::stdout(), &output).expect("Stdout writing error");
            Ok(())
        }
        Err(error) => {
            std::io::stderr()
                .write_all(error.to_string().as_bytes())
                .expect("Stderr writing error");
            Err(error)
        }
    }
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
    let process = command.spawn().map_err(|error| {
        anyhow::anyhow!("{:?} subprocess spawning error: {:?}", executable, error)
    })?;

    let stdin = process
        .stdin
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stdin getting error", executable))?;
    serde_json::to_writer(stdin, &input).map_err(|error| {
        anyhow::anyhow!(
            "{:?} subprocess stdin writing error: {:?}",
            executable,
            error
        )
    })?;
    let result = process.wait_with_output().map_err(|error| {
        anyhow::anyhow!("{:?} subprocess waiting error: {:?}", executable, error)
    })?;
    if !result.status.success() {
        anyhow::bail!("{}", String::from_utf8_lossy(result.stderr.as_slice()),);
    }

    let output: O =
        era_compiler_common::deserialize_from_slice(result.stdout.as_slice()).map_err(|error| {
            anyhow::anyhow!(
                "{:?} subprocess stdout parsing error: {}",
                executable,
                error,
            )
        })?;
    Ok(output)
}
