use predicates::prelude::*;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_BYTECODE_PATH, "--disassemble"];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &["--disassemble", "anyarg"];

    let result = common::execute_zkvyper(args)?;
    result.failure();

    Ok(())
}

#[test]
fn excess_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--disassemble", common::TEST_BYTECODE_PATH, "--overwrite"];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed in disassembler mode.",
    ));

    Ok(())
}
