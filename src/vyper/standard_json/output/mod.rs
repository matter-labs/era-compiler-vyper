//!
//! The `vyper --standard-json` output representation.
//!

pub mod contract;
pub mod error;

use std::collections::BTreeMap;

use serde::Deserialize;
use sha3::digest::FixedOutput;
use sha3::Digest;

use crate::project::contract::vyper::Contract as VyperContract;
use crate::project::Project;

use self::contract::Contract;
use self::error::Error;

///
/// The `vyper --standard-json` output representation.
///
/// Unlike in the Solidity compiler, it is not passed up to the hardhat plugin, but only used here
/// internally to reduce the number of calls to the `vyper` subprocess.
///
#[derive(Debug, Deserialize)]
pub struct Output {
    /// The file-contract hashmap.
    #[serde(rename = "contracts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<BTreeMap<String, BTreeMap<String, Contract>>>,
    /// The compilation errors and warnings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    /// The `vyper` compiler long version.
    #[serde(rename = "compiler")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub long_version: Option<String>,
}

impl Output {
    ///
    /// Converts the `vyper` JSON output into a convenient project representation.
    ///
    pub fn try_into_project(mut self, version: &semver::Version) -> anyhow::Result<Project> {
        let files = match self.files.take() {
            Some(files) => files,
            None => {
                anyhow::bail!(
                    "{}",
                    self.errors
                        .as_ref()
                        .map(|errors| serde_json::to_string_pretty(errors).expect("Always valid"))
                        .unwrap_or_else(|| "Unknown project assembling error".to_owned())
                );
            }
        };

        let mut project_contracts = BTreeMap::new();
        let mut source_code_hasher = sha3::Keccak256::new();
        for (path, file) in files.into_iter() {
            for (name, contract) in file.into_iter() {
                let full_path = format!("{path}:{name}");

                source_code_hasher.update(
                    contract
                        .source_code
                        .expect("Must be set at this point")
                        .as_bytes(),
                );

                let project_contract = VyperContract::new(
                    version.to_owned(),
                    contract.metadata,
                    contract.ir,
                    contract.evm.abi,
                );
                project_contracts.insert(full_path, project_contract.into());
            }
        }
        let source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD] =
            source_code_hasher.finalize_fixed().into();

        Ok(Project::new(
            version.to_owned(),
            source_code_hash,
            project_contracts,
        ))
    }
}
