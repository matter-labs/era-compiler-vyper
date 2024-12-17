use crate::common;
use predicates::prelude::*;

#[test]
fn run_with_disable_vyper_optimizer() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--disable-vyper-optimizer",
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_only_with_disable_vyper_optimizer() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--disable-vyper-optimizer"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

#[test]
fn run_with_duplicate_disable_vyper_optimizer() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--disable-vyper-optimizer",
        common::TEST_GREETER_CONTRACT_PATH,
        "--disable-vyper-optimizer",
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}
