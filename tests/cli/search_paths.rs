use crate::common;
use predicates::prelude::*;

#[test]
fn run_with_search_paths() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--search-paths",
        "tests/cli",
        "tests/unit",
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}
