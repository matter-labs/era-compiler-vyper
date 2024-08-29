use crate::{cli, common};
use predicates::prelude::*;

/// id1959
#[test]
fn test_suppress_warnings_with_specific_option() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        cli::TEST_TX_ORIGIN_CONTRACT_PATH,
        "--suppress-warnings",
        "txorigin",
    ];
    let result = cli::execute_zkvyper(args)?;

    result
        .success()
        .stdout(predicate::str::contains("0x"))
        .stderr(predicate::str::contains("Warning").not());

    Ok(())
}

/// id1959:I
#[test]
fn test_suppress_warnings_without_specific_option() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[cli::TEST_TX_ORIGIN_CONTRACT_PATH, "--suppress-warnings"];
    let result = cli::execute_zkvyper(args)?;

    result
        .success()
        .stdout(predicate::str::contains("0x"))
        .stderr(predicate::str::contains("Warning"));

    Ok(())
}
