//!
//! The Vyper project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use crate::vyper::combined_json::contract::Contract as CombinedJsonContract;
use crate::vyper::combined_json::CombinedJson;

use self::contract::Contract;

///
/// The Vyper project build.
///
#[derive(Debug, Default)]
pub struct Build {
    /// The contract data,
    pub contracts: BTreeMap<String, Contract>,
}

impl Build {
    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        for (contract_path, contract) in self.contracts.into_iter() {
            contract.write_to_directory(
                output_directory,
                PathBuf::from(contract_path).as_path(),
                overwrite,
            )?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the combined JSON.
    ///
    pub fn write_to_combined_json(
        self,
        combined_json: &mut CombinedJson,
        zkvyper_version: &semver::Version,
        output_assembly: bool,
    ) -> anyhow::Result<()> {
        for (path, contract) in self.contracts.into_iter() {
            let combined_json_contract =
                combined_json
                    .contracts
                    .iter_mut()
                    .find_map(|(json_path, contract)| {
                        let path = PathBuf::from(path.as_str())
                            .normalize()
                            .expect("Path normalization error");
                        let json_path = PathBuf::from(json_path.as_str())
                            .normalize()
                            .expect("Path normalization error");

                        if path.ends_with(json_path) {
                            Some(contract)
                        } else {
                            None
                        }
                    });

            match combined_json_contract {
                Some(combined_json_contract) => {
                    contract.write_to_combined_json(combined_json_contract)?
                }
                None if path.as_str() == crate::r#const::MINIMAL_PROXY_CONTRACT_NAME => {
                    combined_json.contracts.insert(
                        path,
                        CombinedJsonContract::new_minimal_proxy(output_assembly),
                    );
                }
                None => {
                    anyhow::bail!("Contract `{path}` not found in the project");
                }
            }
        }

        combined_json.zk_version = Some(zkvyper_version.to_string());

        Ok(())
    }
}
