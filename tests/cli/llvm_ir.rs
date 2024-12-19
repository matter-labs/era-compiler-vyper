use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--llvm-ir", common::TEST_LLVM_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn invalid() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--llvm-ir", common::TEST_GREETER_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}

#[test]
fn not_found() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--llvm-ir", "unknown"];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("reading error"));

    Ok(())
}
