use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_with_eravm_assembly_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH, "--eravm-assembly"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn run_only_with_eravm_assembly_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--eravm-assembly"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided"));

    Ok(())
}

#[test]
fn run_with_duplicate_eravm_assembly_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--eravm-assembly",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: the argument '--eravm-assembly' cannot be used multiple times",
    ));

    Ok(())
}
