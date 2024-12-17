use crate::common;
use predicates::prelude::*;

/// List of optimization arguments
const OPTIMIZATION_ARGS: [&str; 6] = ["0", "1", "2", "3", "s", "z"];

#[test]
fn test_optimization_valid_flags() -> anyhow::Result<()> {
    let _ = common::setup();

    for arg in OPTIMIZATION_ARGS.iter() {
        let args = &[common::TEST_GREETER_CONTRACT_PATH, &format!("-O{arg}")];
        let result = common::execute_zkvyper(args)?;

        result.success().stdout(predicate::str::contains("0x"));
    }

    Ok(())
}

#[test]
fn test_optimization_missing_contract() -> anyhow::Result<()> {
    let _ = common::setup();

    for arg in OPTIMIZATION_ARGS.iter() {
        let opt_param = format!("-O{arg}");
        let args = &[opt_param.as_str()];
        let result = common::execute_zkvyper(args)?;

        result
            .failure()
            .stderr(predicate::str::contains("No input files provided"));
    }

    Ok(())
}

#[test]
fn test_optimization_duplicate_flags() -> anyhow::Result<()> {
    let _ = common::setup();

    for arg in OPTIMIZATION_ARGS.iter() {
        let args = &[
            common::TEST_GREETER_CONTRACT_PATH,
            &format!("-O{arg}"),
            &format!("-O{arg}"),
        ];
        let result = common::execute_zkvyper(args)?;

        result.failure().stderr(predicate::str::contains(
            "error: the argument '--optimization <OPTIMIZATION>' cannot be used multiple times",
        ));
    }

    Ok(())
}
