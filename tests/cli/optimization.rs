use predicates::prelude::*;
use test_case::test_case;

use crate::common;

#[test_case('0')]
#[test_case('1')]
#[test_case('2')]
#[test_case('3')]
#[test_case('s')]
#[test_case('z')]
fn default(level: char) -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_GREETER_CONTRACT_PATH, &format!("-O{level}")];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--optimization",
        "3",
        "--eravm-assembly",
        common::TEST_GREETER_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "LLVM optimizations are not supported in EraVM assembly mode.",
    ));

    Ok(())
}
