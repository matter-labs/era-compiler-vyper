use crate::{cli, common};
use predicates::prelude::*;

/// id1942
#[test]
fn run_with_vyper() -> anyhow::Result<()> {
    let _ = common::setup();
    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--vyper", &vyper];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1942:II
#[test]
fn run_with_vyper_empty_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--vyper"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "requires a value but none was supplied",
    ));

    Ok(())
}

/// id1942:III
#[test]
fn run_with_vyper_wrong_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--vyper", ".."];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("not found")));

    Ok(())
}
