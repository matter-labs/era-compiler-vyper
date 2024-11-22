use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_with_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-ir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_only_with_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--llvm-ir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

#[test]
fn run_with_duplicate_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-ir", "--llvm-ir"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: the argument '--llvm-ir' cannot be used multiple times",
    ));

    Ok(())
}

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
