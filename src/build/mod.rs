//!
//! The Vyper project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use crate::vyper::combined_json::contract::Contract as CombinedJsonContract;
use crate::vyper::combined_json::CombinedJson;
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
    pub fn write_to_terminal(
        self,
        format: Option<&str>,
        vyper_path: Option<&str>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        enable_decimals: bool,
    ) -> anyhow::Result<()> {
        for (path, contract) in self.contracts.into_iter() {
            for warning in contract.warnings.iter() {
                writeln!(std::io::stderr(), "\n{warning}")?;
            }

            writeln!(std::io::stdout(), "Contract `{path}`:")?;

            let bytecode_string = hex::encode(contract.build.bytecode);
            writeln!(std::io::stdout(), "0x{bytecode_string}")?;

            if let Some(format) = format {
                let vyper = VyperCompiler::new(
                    vyper_path.unwrap_or(VyperCompiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                let extra_output = vyper.extra_output(
                    PathBuf::from(path).as_path(),
                    evm_version,
                    enable_decimals,
                    format,
                )?;
                writeln!(std::io::stdout(), "\n{extra_output}")?;
            }
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
