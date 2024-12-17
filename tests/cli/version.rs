use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--version"];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Vyper compiler for ZKsync"));

    Ok(())
}

#[test]
fn excess_args() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--version", common::TEST_GREETER_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed while getting the compiler version.",
    ));

    Ok(())
}
