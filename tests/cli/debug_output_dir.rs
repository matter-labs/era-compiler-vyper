use crate::{cli, common};
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// id1976:I
#[test]
fn run_contract_with_debug_output_dir() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .success()
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Ensure output directory is created
    assert!(tmp_dir_zkvyper.path().exists());

    // Ensure it contains expected filenames
    let expected_substrings = [
        cli::LLVM_IR_EXTENSION,
        cli::LLVM_IR_OPTIMIZED_EXTENSION,
        cli::LLVM_IR_UNOPTIMIZED_EXTENSION,
        cli::ERAVM_ASSEMBLY_EXTENSION,
    ];
    let filenames = fs::read_dir(tmp_dir_zkvyper.path())?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    assert!(filenames.iter().all(|filename| expected_substrings
        .iter()
        .any(|substring| filename.contains(substring))));

    Ok(())
}

/// id1976:II
#[test]
fn run_without_contract_with_debug_output_dir() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "No input files provided",
    ));

    Ok(())
}

/// id1976:III
#[test]
fn run_with_debug_output_dir_no_folder_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--debug-output-dir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("requires a value"));

    Ok(())
}

/// id1977
#[test]
fn run_with_double_debug_output_dir_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}
