//!
//! The `vyper --standard-json` output.
//!

pub mod contract;
pub mod error;

use std::collections::BTreeMap;

use serde::Deserialize;
use sha3::digest::FixedOutput;
use sha3::Digest;

use crate::metadata::Metadata as SourceMetadata;
use crate::project::contract::vyper::ast::AST as VyperAST;
use crate::project::contract::vyper::Contract as VyperContract;
use crate::project::contract::Contract as ProjectContract;
use crate::project::Project;

use self::contract::Contract;
use self::error::Error;

///
/// The `vyper --standard-json` output.
///
/// Unlike in the Solidity compiler, it is not passed up to the hardhat plugin, but only used here
/// internally to reduce the number of calls to the `vyper` subprocess.
///
#[derive(Debug, Deserialize)]
pub struct Output {
    /// The contracts hashmap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<BTreeMap<String, BTreeMap<String, Contract>>>,
    /// The source code hashmap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<BTreeMap<String, serde_json::Value>>,
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
    /// Converts the `vyper` JSON output into a convenient project.
    ///
    pub fn try_into_project(mut self, version: &semver::Version) -> anyhow::Result<Project> {
        let files = match self.contracts.take() {
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

        let mut project_contracts: BTreeMap<String, ProjectContract> = BTreeMap::new();
        for (path, file) in files.into_iter() {
            for (name, contract) in file.into_iter() {
                let full_path = format!("{path}:{name}");

                let ast = self
                    .sources
                    .as_ref()
                    .and_then(|sources| sources.get(&path).cloned())
                    .ok_or_else(|| anyhow::anyhow!("No AST for contract {}", full_path))?;
                let ast = VyperAST::new(full_path.clone(), ast);

                let project_contract = VyperContract::new(
                    version.to_owned(),
                    contract.source_code.expect("Must be set by the tester"),
                    SourceMetadata::default(),
                    contract.ir,
                    contract.evm.abi,
                    ast,
                );
                project_contracts.insert(full_path, project_contract.into());
            }
        }

        let mut source_code_hasher = sha3::Keccak256::new();
        for (_path, contract) in project_contracts.iter() {
            source_code_hasher.update(contract.source_code().as_bytes());
        }
        let source_code_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD] =
            source_code_hasher.finalize_fixed().into();

        Ok(Project::new(
            version.to_owned(),
            source_code_hash,
            project_contracts,
        ))
    }
}
