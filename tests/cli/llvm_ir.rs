use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_LLVM_CONTRACT_PATH, "--llvm-ir"];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn invalid_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--llvm-ir"];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}
