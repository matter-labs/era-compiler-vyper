use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--vyper",
        vyper.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--vyper",
        "invalid_path",
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure();

    Ok(())
}

#[test]
fn llvm_ir_mode() -> anyhow::Result<()> {
    let _ = common::setup();

    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let args = &[
        "--vyper",
        vyper.as_str(),
        "--llvm-ir",
        common::TEST_GREETER_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "`vyper` is not used in LLVM IR and EraVM assembly modes.",
    ));

    Ok(())
}

#[test]
fn eravm_assembly_mode() -> anyhow::Result<()> {
    let _ = common::setup();

    let vyper = common::get_vyper_compiler(&semver::Version::new(0, 4, 0))?.executable;
    let args = &[
        "--vyper",
        vyper.as_str(),
        "--eravm-assembly",
        common::TEST_GREETER_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "`vyper` is not used in LLVM IR and EraVM assembly modes.",
    ));

    Ok(())
}
