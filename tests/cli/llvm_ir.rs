use crate::{cli, common};
use predicates::prelude::*;

/// id1963
#[test]
fn run_with_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-ir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1963:II
#[test]
fn run_only_with_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--llvm-ir"];

    // Execute zkvyper command
    // TODO: change success() to failure() after CPR-1804 fix
    let result = cli::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("No input sources provided"));

    Ok(())
}

/// id1964
#[test]
fn run_with_double_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-ir", "--llvm-ir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("was provided more than once"));

    Ok(())
}

/// id1965
#[test]
fn run_with_incompatible_contract_and_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--llvm-ir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}
