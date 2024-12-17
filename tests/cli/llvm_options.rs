use crate::common;
use predicates::prelude::*;

#[test]
fn run_zkvyper_with_llvm_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}
