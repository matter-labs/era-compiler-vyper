//!
//! The Vyper project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use crate::vyper::combined_json::CombinedJson;
use crate::vyper::selection::Selection as VyperSelection;
use crate::vyper::Compiler as VyperCompiler;

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
    pub fn write_to_terminal(self, selection: &[VyperSelection]) -> anyhow::Result<()> {
        for (path, contract) in self.contracts.into_iter() {
            contract.write_to_terminal(path, selection)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        selection: &[VyperSelection],
        output_directory: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        std::fs::create_dir_all(output_directory)?;

        for (contract_path, contract) in self.contracts.into_iter() {
            contract.write_to_directory(
                selection,
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
            .map(|(path, contract)| {
                let contract_path = PathBuf::from(path.as_str());
                let contract_path = contract_path
                    .normalize()
                    .map(|path| path.into_path_buf())
                    .unwrap_or(contract_path);

                let contract_path = if version < Some(&VyperCompiler::FIRST_VERSION_ABSOLUTE_PATHS)
                {
                    std::env::current_dir()
                        .map_err(anyhow::Error::from)
                        .and_then(|path| crate::path_to_posix(path.as_path()))
                        .and_then(|path| {
                            contract_path
                                .strip_prefix(path)
                                .map_err(anyhow::Error::from)
                        })
                        .unwrap_or(contract_path.as_path())
                } else {
                    contract_path.as_path()
                };

                (
                    contract_path.to_string_lossy().to_string(),
                    contract.into_combined_json(),
                )
            })
            .collect();

        CombinedJson::new(contracts, version, zkvyper_version)
    }
}
