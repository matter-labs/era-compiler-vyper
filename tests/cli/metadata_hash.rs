use crate::{cli, common};
use predicates::prelude::*;

/// id1941
#[test]
fn run_with_metadata_hash_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--metadata-hash=none"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1941:II
#[test]
fn run_with_incomplete_metadata_hash_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--metadata-hash"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("requires a value"));

    Ok(())
}

/// id1941:III
#[test]
fn run_only_with_metadata_hash_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--metadata-hash=none"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

/// id1975
#[test]
fn run_with_double_metadata_hash_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--metadata-hash=none",
        "--metadata-hash",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("was provided more than once"));

    Ok(())
}
