use crate::common;
use predicates::prelude::*;

#[test]
fn run_with_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--llvm-verify-each"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_only_with_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--llvm-verify-each"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

#[test]
fn run_with_duplicate_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--llvm-verify-each",
        "--llvm-verify-each",
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: the argument '--llvm-verify-each' cannot be used multiple times",
    ));

    Ok(())
}

#[test]
fn run_with_incompatible_contract_and_llvm_verify_each() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_LLVM_CONTRACT_PATH, "--llvm-verify-each"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("vyper error"));

    Ok(())
}
