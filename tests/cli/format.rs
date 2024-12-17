use predicates::prelude::*;
use test_case::test_case;

use crate::common;

#[test_case("combined_json")]
#[test_case("ir_json")]
#[test_case("ast")]
#[test_case("abi")]
#[test_case("method_identifiers")]
#[test_case("layout")]
#[test_case("userdoc")]
#[test_case("devdoc")]
#[test_case("eravm_assembly")]
#[test_case("project_metadata")]
fn default(selector: &str) -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[common::TEST_GREETER_CONTRACT_PATH, "-f", selector];

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
