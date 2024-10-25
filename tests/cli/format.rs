use crate::{cli, common};
use predicates::prelude::*;

/// id1988
#[test]
fn run_with_format_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let format_args = [
        "combined_json",
        "ir_json",
        "ast",
        "abi",
        "method_identifiers",
        "layout",
        "userdoc",
        "devdoc",
        "eravm_assembly",
    ];

    for format in format_args.iter() {
        let args = &[cli::TEST_VYPER_CONTRACT_PATH, "-f", format];

        // Execute zkvyper command
        let result = cli::execute_zkvyper(args)?;
        result.success();
    }

    Ok(())
}

/// id1989
#[test]
fn run_with_unsupported_format() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_VYPER_CONTRACT_PATH, "-f", "llvm"];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Unknown selection flag"));

    Ok(())
}

/// id1990
#[test]
fn run_with_double_format_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let format_args = ["combined_json", "ir_json"];
    let args = &[
        cli::TEST_VYPER_CONTRACT_PATH,
        "-f",
        format_args[0],
        "-f",
        format_args[1],
    ];

    // Execute zkvyper command
    let result = cli::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}
