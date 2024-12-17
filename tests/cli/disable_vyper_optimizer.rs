use crate::common;
use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--disable-vyper-optimizer",
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}
