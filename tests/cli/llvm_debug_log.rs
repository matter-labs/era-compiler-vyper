use crate::{cli, common};
use predicates::prelude::*;

/// id1980
#[test]
fn run_with_llvm_debug_logging() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--llvm-debug-logging"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1980:II
#[test]
fn run_only_with_llvm_debug_logging() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--llvm-debug-logging"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("arguments are required"));

    Ok(())
}

/// id1981
#[test]
fn run_with_double_llvm_debug_logging() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--llvm-debug-logging",
        "--llvm-debug-logging",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("was provided more than once"));

    Ok(())
}

/// id1965
#[test]
fn run_with_incompatible_contract_and_llvm_debug_logging() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-debug-logging"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("vyper error"));

    Ok(())
}
