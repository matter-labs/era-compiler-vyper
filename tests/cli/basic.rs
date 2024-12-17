use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_GREETER_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    let zkvyper_status = result
        .success()
        .stdout(predicate::str::contains("0x"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let vyper_result = common::execute_vyper(args)?;
    vyper_result.code(zkvyper_status);

    Ok(())
}
