use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_GREETER_CONTRACT_PATH, "--threads", "1"];

    let result = common::execute_zkvyper(args)?;
    result.success();

    Ok(())
}
