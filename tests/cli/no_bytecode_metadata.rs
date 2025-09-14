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
        "--no-bytecode-metadata",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a165").not())
        .stdout(predicate::str::ends_with("0023").not());

    Ok(())
}

#[test]
fn ipfs_vyper() -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = MetadataHashType::IPFS.to_string();
    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-bytecode-metadata",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("004c").not());

    Ok(())
}

#[test]
fn ipfs_llvm_ir() -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = MetadataHashType::IPFS.to_string();
    let args = &[
        "--llvm-ir",
        common::TEST_LLVM_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-bytecode-metadata",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("004c").not());

    Ok(())
}

#[test]
fn ipfs_eravm_assembly() -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = MetadataHashType::IPFS.to_string();
    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-bytecode-metadata",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("004c").not());

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
        "--no-bytecode-metadata",
    ];

    let result = common::execute_zkvyper(args)?;
    result
        .success()
        .stdout(predicate::str::contains("0x"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("004c").not())
        .stderr(predicate::str::contains(
            "`keccak256` metadata hash type is deprecated. Please use `ipfs` instead.",
        ));

    Ok(())
}
