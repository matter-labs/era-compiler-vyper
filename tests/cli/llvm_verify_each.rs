use crate::{cli, common};
use predicates::prelude::*;

/// id1972
#[test]
fn run_with_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--llvm-verify-each"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1972:II
#[test]
fn run_only_with_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--llvm-verify-each"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

/// id1973
#[test]
fn run_with_double_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--llvm-verify-each",
        "--llvm-verify-each",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("was provided more than once"));

    Ok(())
}

/// id1974
#[test]
fn run_with_incompatible_contract_and_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-verify-each"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("vyper error"));

    Ok(())
}
