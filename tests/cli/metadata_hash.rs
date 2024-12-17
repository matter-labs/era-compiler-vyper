use crate::common;
use predicates::prelude::*;

#[test]
fn run_with_metadata_hash_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--metadata-hash=none"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_with_incomplete_metadata_hash_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--metadata-hash"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: a value is required for '--metadata-hash <METADATA_HASH>' but none was supplied",
    ));

    Ok(())
}

#[test]
fn run_only_with_metadata_hash_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--metadata-hash=none"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

#[test]
fn run_with_duplicate_metadata_hash_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--metadata-hash=none",
        "--metadata-hash",
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: a value is required for '--metadata-hash <METADATA_HASH>' but none was supplied",
    ));

    Ok(())
}
