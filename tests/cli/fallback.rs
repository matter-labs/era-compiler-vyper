use crate::common;
use predicates::prelude::*;

#[test]
fn run_with_fallback_oz_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--fallback-Oz"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_only_with_fallback_oz_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--fallback-Oz"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}
