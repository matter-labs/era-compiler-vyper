use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn invalid() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--eravm-assembly", common::TEST_GREETER_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot parse operand"));

    Ok(())
}

#[test]
fn not_found() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--eravm-assembly", "unknown"];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("reading error"));

    Ok(())
}
