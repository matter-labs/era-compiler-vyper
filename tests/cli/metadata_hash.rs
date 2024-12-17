use predicates::prelude::*;
use test_case::test_case;

use era_compiler_common::HashType;

use crate::common;

#[test_case(HashType::None)]
#[test_case(HashType::Keccak256)]
#[test_case(HashType::Ipfs)]
fn default(hash_type: HashType) -> anyhow::Result<()> {
    let _ = common::setup();

    let hash_type = match hash_type {
        HashType::None => "none",
        HashType::Keccak256 => "keccak256",
        HashType::Ipfs => "ipfs",
    };

    let args = &[
        common::TEST_GREETER_CONTRACT_PATH,
        "--metadata-hash",
        hash_type,
    ];

    let result = common::execute_zkvyper(args)?;
    result.success().stdout(predicate::str::contains("0x"));

    Ok(())
}
