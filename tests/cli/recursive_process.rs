use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--recursive-process"];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Stdin reading error"));

    Ok(())
}

#[test]
fn excess_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--recursive-process", common::TEST_GREETER_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed in recursive mode.",
    ));

    Ok(())
}
