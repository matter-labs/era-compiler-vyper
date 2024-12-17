use crate::common;
use predicates::prelude::*;

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
        let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", format];

        // Execute zkvyper command
        let result = common::execute_zkvyper(args)?;
        result.success();
    }

    Ok(())
}

#[test]
fn run_with_unsupported_format() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", "llvm"];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Unknown selection flag"));

    Ok(())
}

#[test]
fn run_with_duplicate_format_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let format_args = ["combined_json", "ir_json"];
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "-f",
        format_args[0],
        "-f",
        format_args[1],
    ];

    // Execute zkvyper command
    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}
