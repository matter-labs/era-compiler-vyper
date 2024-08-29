use crate::{cli, common};
use predicates::prelude::*;

/// List of optimization arguments
const OPTIMIZATION_ARGS: [&str; 6] = ["0", "1", "2", "3", "s", "z"];

/// id1937
#[test]
fn test_optimization_valid_flags() -> anyhow::Result<()> {
    let _ = common::setup();

    for arg in OPTIMIZATION_ARGS.iter() {
        let args = &[cli::TEST_VYPER_CONTRACT_PATH, &format!("-O{arg}")];
        let result = cli::execute_zkvyper(args)?;

        result.success().stdout(predicate::str::contains("0x"));
    }

    Ok(())
}

/// id1937:I
#[test]
fn test_optimization_missing_contract() -> anyhow::Result<()> {
    let _ = common::setup();

    for arg in OPTIMIZATION_ARGS.iter() {
        let opt_param = format!("-O{arg}");
        let args = &[opt_param.as_str()];
        let result = cli::execute_zkvyper(args)?;

        result.failure().stderr(predicate::str::contains(
            "the following arguments are required",
        ));
    }

    Ok(())
}

/// id1978
#[test]
fn test_optimization_duplicate_flags() -> anyhow::Result<()> {
    let _ = common::setup();

    for arg in OPTIMIZATION_ARGS.iter() {
        let args = &[
            cli::TEST_VYPER_CONTRACT_PATH,
            &format!("-O{arg}"),
            &format!("-O{arg}"),
        ];
        let result = cli::execute_zkvyper(args)?;

        result
            .failure()
            .stderr(predicate::str::contains("provided more than once"));
    }

    Ok(())
}
