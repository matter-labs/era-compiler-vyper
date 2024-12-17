use crate::common;
use predicates::prelude::*;

#[test]
fn run_zkvyper_with_disassemble() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_BYTECODE_PATH, "--disassemble"];
    let invalid_args = &["--disassemble", "anyarg"];

    let result = common::execute_zkvyper(args)?;
    let invalid_result = common::execute_zkvyper(invalid_args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));
    invalid_result.failure();

    Ok(())
}
