use crate::{cli, common};
use predicates::prelude::*;
use tempfile::TempDir;

/// id1936
#[test]
fn run_zkvyper_without_any_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    let zkvyper_status = result
        .failure()
        .stderr(predicate::str::contains("Vyper compiler for ZKsync"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with vyper
    // Use `ge` predicate to check if zkvyper exit code is greater than or equal to vyper exit code
    // because vyper exit code is 2, but zkvyper exit code is 1
    cli::execute_vyper(args)?.code(predicate::ge(zkvyper_status));

    Ok(())
}

/// id1978
#[test]
fn default_run_without_input_files() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["-f", "ast"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    let zkvyper_status = result
        .failure()
        .stderr(predicate::str::contains("No input files provided"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with vyper
    // Use `ge` predicate to check if zkvyper exit code is greater than or equal to vyper exit code
    // because vyper exit code is 2, but zkvyper exit code is 1
    let vyper_result = cli::execute_vyper(args)?;
    vyper_result.code(predicate::ge(zkvyper_status));

    Ok(())
}

/// id1978
#[test]
fn default_run_with_a_contract_only() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    let zkvyper_status = result
        .success()
        .stdout(predicate::str::contains("0x"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with vyper
    let vyper_result = cli::execute_vyper(args)?;
    vyper_result.code(zkvyper_status);

    Ok(())
}

/// id1983
#[test]
fn default_run_command_from_help() -> anyhow::Result<()> {
    let _ = common::setup();
    let output_dir = TempDir::new()?;
    let bin_output_file = output_dir.path().join(cli::VYPER_BIN_OUTPUT_NAME);

    let zkvyper_args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "-o",
        output_dir.path().to_str().unwrap(),
        "-f",
        "eravm_assembly",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(zkvyper_args)?;
    let zkvyper_status = result
        .success()
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Ensure output directory is created
    assert!(output_dir.path().exists());

    // Ensure output file is created and is not empty
    assert!(bin_output_file.exists());
    assert!(!cli::is_file_empty(&bin_output_file.to_str().unwrap())?);

    // Compare with vyper
    let vyper_args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "-o",
        bin_output_file.to_str().unwrap(),
    ];
    let vyper_result = cli::execute_vyper(vyper_args)?;
    vyper_result.code(zkvyper_status);

    Ok(())
}
