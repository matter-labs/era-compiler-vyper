use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_with_search_paths() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "--search-paths",
        "tests/cli",
        "tests/regression",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}
