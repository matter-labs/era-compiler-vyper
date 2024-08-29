use crate::{cli, common};
use predicates::prelude::*;

/// id1961
#[test]
fn run_with_eravm_assembly_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH, "--eravm-assembly"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

/// id1963:II
#[test]
fn run_only_with_eravm_assembly_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--eravm-assembly"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .success()
        .stderr(predicate::str::contains("No input sources provided"));

    Ok(())
}

/// id1962
#[test]
fn run_with_double_eravm_assembly_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--eravm-assembly",
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "The argument '--eravm-assembly' was provided more than once",
    ));

    Ok(())
}
