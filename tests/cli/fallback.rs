use crate::{cli, common};
use predicates::prelude::*;

/// id1938
#[test]
fn run_with_fallback_oz_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "--fallback-Oz"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1938:I
#[test]
fn run_only_with_fallback_oz_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--fallback-Oz"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "the following arguments are required",
    ));

    Ok(())
}
