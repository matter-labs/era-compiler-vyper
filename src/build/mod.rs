//!
//! The Vyper project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(self) -> anyhow::Result<()> {
        for (path, contract) in self.contracts.into_iter() {
            contract.write_to_terminal(path)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        std::fs::create_dir_all(output_directory)?;

        for (contract_path, contract) in self.contracts.into_iter() {
            for warning in contract.warnings.iter() {
                writeln!(std::io::stderr(), "\n{warning}")?;
            }

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
    pub fn into_combined_json(
        self,
        version: Option<&semver::Version>,
        zkvyper_version: &semver::Version,
    ) -> CombinedJson {
        let contracts = self
            .contracts
            .into_iter()
            .map(|(path, contract)| (path, contract.into_combined_json()))
            .collect();

        CombinedJson::new(contracts, version, zkvyper_version)
    }
}
