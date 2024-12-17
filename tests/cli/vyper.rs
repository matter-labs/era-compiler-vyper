use crate::common;
use predicates::prelude::*;

#[test]
fn run_with_vyper() -> anyhow::Result<()> {
    let _ = common::setup();
    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--vyper", &vyper];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_with_vyper_empty_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--vyper"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: a value is required for '--vyper <VYPER>' but none was supplied",
    ));

    Ok(())
}

#[test]
fn run_with_vyper_wrong_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--vyper", ".."];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("not found")));

    Ok(())
}
