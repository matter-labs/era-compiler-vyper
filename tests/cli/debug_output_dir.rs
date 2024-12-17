use crate::common;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn run_contract_with_debug_output_dir() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
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
        common::LLVM_IR_EXTENSION,
        common::LLVM_IR_OPTIMIZED_EXTENSION,
        common::LLVM_IR_UNOPTIMIZED_EXTENSION,
        common::ERAVM_ASSEMBLY_EXTENSION,
    ];
    let filenames = fs::read_dir(tmp_dir_zkvyper.path())?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    assert!(filenames.iter().all(|filename| expected_substrings
        .iter()
        .any(|substring| filename.contains(substring))));

    Ok(())
}

#[test]
fn run_without_contract_with_debug_output_dir() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

#[test]
fn run_with_debug_output_dir_no_folder_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--debug-output-dir"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: a value is required for '--debug-output-dir <DEBUG_OUTPUT_DIR>' but none was supplied"));

    Ok(())
}

#[test]
fn run_with_duplicate_debug_output_dir_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zkvyper = TempDir::new()?;
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
        "--debug-output-dir",
        tmp_dir_zkvyper.path().to_str().unwrap(),
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}
