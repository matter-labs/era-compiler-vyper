use predicates::prelude::*;

use era_compiler_vyper::VyperSelector;

use crate::common;

#[test]
fn default() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_GREETER_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    let zkvyper_status = result
        .success()
        .stdout(predicate::str::contains("0x"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let vyper_result = common::execute_vyper(args)?;
    vyper_result.code(zkvyper_status);

    Ok(())
}

#[test]
fn with_proxy() -> anyhow::Result<()> {
    let _ = common::setup();

    let format = [
        VyperSelector::IRJson,
        VyperSelector::AST,
        VyperSelector::ABI,
        VyperSelector::MethodIdentifiers,
        VyperSelector::Layout,
        VyperSelector::UserDocumentation,
        VyperSelector::DeveloperDocumentation,
    ]
    .into_iter()
    .map(|selection| selection.to_string())
    .collect::<Vec<String>>()
    .join(",");

    let args = &[
        common::TEST_CREATE_MINIMAL_PROXY_TO_CONTRACT_PATH,
        "-f",
        format.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    let zkvyper_status = result
        .success()
        .stdout(predicate::str::contains("0x"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let vyper_result = common::execute_vyper(args)?;
    vyper_result.code(zkvyper_status);

    Ok(())
}

#[test]
fn with_proxy_zkvyper() -> anyhow::Result<()> {
    let _ = common::setup();

    let format = [
        VyperSelector::IRJson,
        VyperSelector::AST,
        VyperSelector::ABI,
        VyperSelector::MethodIdentifiers,
        VyperSelector::Layout,
        VyperSelector::UserDocumentation,
        VyperSelector::DeveloperDocumentation,
        VyperSelector::EraVMAssembly,
        VyperSelector::ProjectMetadata,
    ]
    .into_iter()
    .map(|selection| selection.to_string())
    .collect::<Vec<String>>()
    .join(",");

    let args = &[
        common::TEST_CREATE_MINIMAL_PROXY_TO_CONTRACT_PATH,
        "-f",
        format.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}

#[test]
fn with_warnings() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_TX_ORIGIN_CONTRACT_PATH];

    let result = common::execute_zkvyper(args)?;
    let zkvyper_status = result
        .success()
        .stdout(predicate::str::contains("0x"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let vyper_result = common::execute_vyper(args)?;
    vyper_result.code(zkvyper_status);

    Ok(())
}

#[test]
fn without_input_files() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input files provided."));

    let vyper_result = common::execute_vyper(args)?;
    vyper_result.code(2);

    Ok(())
}

#[test]
fn multiple_modes() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--llvm-ir",
        "--eravm-assembly",
    ];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "Only one mode is allowed at the same time: format, LLVM IR, EraVM assembly, disassembler.",
    ));

    Ok(())
}
