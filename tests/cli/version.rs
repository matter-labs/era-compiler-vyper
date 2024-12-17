use crate::common;
use predicates::prelude::*;

#[test]
fn test_version() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--version"];
    let result = common::execute_zkvyper(args)?;

    result
        .success()
        .stdout(predicate::str::contains("Vyper compiler for ZKsync"));

    Ok(())
}

#[test]
fn test_version_with_extra_args() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--version", common::TEST_GREETER_CONTRACT_PATH];
    let result = common::execute_zkvyper(args)?;

    result.failure().stderr(predicate::str::contains(
        "Error: No other options are allowed while getting the compiler version.",
    ));

    Ok(())
}
