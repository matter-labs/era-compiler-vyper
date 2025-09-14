use predicates::prelude::*;

use era_compiler_common::MetadataHashType;

use crate::common;

#[test]
fn none() -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = MetadataHashType::None.to_string();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a165"))
        .stdout(predicate::str::contains("0023"));

    Ok(())
}

#[test]
fn ipfs() -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = MetadataHashType::IPFS.to_string();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264"))
        .stdout(predicate::str::contains("004c"));

    Ok(())
}

#[test]
fn keccak256() -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = MetadataHashType::Keccak256.to_string();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("0x"))
        .stderr(predicate::str::contains(
            "`keccak256` metadata hash type is deprecated. Please use `ipfs` instead.",
        ));

    Ok(())
}
