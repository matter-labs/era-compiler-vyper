use predicates::prelude::*;
use test_case::test_case;

use era_compiler_vyper::VyperSelector;

use crate::common;

#[test_case(VyperSelector::CombinedJson)]
#[test_case(VyperSelector::IRJson)]
#[test_case(VyperSelector::AST)]
#[test_case(VyperSelector::ABI)]
#[test_case(VyperSelector::MethodIdentifiers)]
#[test_case(VyperSelector::Layout)]
#[test_case(VyperSelector::UserDocumentation)]
#[test_case(VyperSelector::DeveloperDocumentation)]
#[test_case(VyperSelector::EraVMAssembly)]
#[test_case(VyperSelector::ProjectMetadata)]
fn default(selector: VyperSelector) -> anyhow::Result<()> {
    let _ = common::setup();

    let selector = selector.to_string();
    let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", selector.as_str()];

    let result = common::execute_zkvyper(args)?;
    result.success();

    Ok(())
}

#[test]
fn all() -> anyhow::Result<()> {
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

    let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", format.as_str()];

    let result = common::execute_zkvyper(args)?;
    result.success();

    Ok(())
}

#[test]
fn unsupported_selector() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", "llvm"];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Unknown selection flag"));

    Ok(())
}

#[test]
fn combined_json_with_other_option() -> anyhow::Result<()> {
    let _ = common::setup();

    let format = [VyperSelector::CombinedJson, VyperSelector::AST]
        .into_iter()
        .map(|selection| selection.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", format.as_str()];

    let result = common::execute_zkvyper(args)?;
    result.failure().stderr(predicate::str::contains(
        "`combined_json` cannot be requested together with other output",
    ));

    Ok(())
}
