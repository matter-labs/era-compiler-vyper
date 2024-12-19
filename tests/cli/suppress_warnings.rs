use predicates::prelude::*;
use test_case::test_case;

use era_compiler_vyper::WarningType;

use crate::common;

#[test_case(WarningType::TxOrigin)]
fn default(warning_type: WarningType) -> anyhow::Result<()> {
    let _ = common::setup();

    let warning_type = warning_type.to_string();
    let args = &[
        common::TEST_TX_ORIGIN_CONTRACT_PATH,
        "--suppress-warnings",
        warning_type.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("0x"))
        .stderr(predicate::str::contains("Warning").not());

    Ok(())
}

#[test]
fn invalid_type() -> anyhow::Result<()> {
    let _ = common::setup();

    let args = &[
        common::TEST_TX_ORIGIN_CONTRACT_PATH,
        "--suppress-warnings",
        "unknown",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("invalid warning type: unknown"));

    Ok(())
}
