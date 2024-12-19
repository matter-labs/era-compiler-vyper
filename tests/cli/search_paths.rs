use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--search-paths",
        "tests/cli",
        "tests/unit",
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}
