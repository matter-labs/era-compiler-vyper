use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        common::TEST_GREETER_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--llvm-ir",
        common::TEST_GREETER_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is not used in LLVM IR and EraVM assembly modes.",
    ));

    Ok(())
}

#[test]
fn eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--eravm-assembly",
        common::TEST_GREETER_CONTRACT_PATH,
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is not used in LLVM IR and EraVM assembly modes.",
    ));

    Ok(())
}
