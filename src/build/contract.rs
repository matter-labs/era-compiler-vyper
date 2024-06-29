//!
//! The Vyper contract build.
//!

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::vyper::combined_json::contract::warning::Warning as CombinedJsonContractWarning;
use crate::vyper::combined_json::contract::Contract as CombinedJsonContract;

///
/// The Vyper contract build.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The LLVM module build.
    pub build: era_compiler_llvm_context::EraVMBuild,
    /// The compilation warnings.
    pub warnings: Vec<CombinedJsonContractWarning>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        build: era_compiler_llvm_context::EraVMBuild,
        warnings: Vec<CombinedJsonContractWarning>,
    ) -> Self {
        Self { build, warnings }
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        contract_path: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let contract_name = Self::contract_name(contract_path.to_str().expect("Always valid"));

        if let Some(assembly) = self.build.assembly {
            let assembly_file_name = format!(
                "{}.{}",
                contract_name,
                era_compiler_common::EXTENSION_ERAVM_ASSEMBLY
            );
            let mut assembly_file_path = output_directory.to_owned();
            assembly_file_path.push(assembly_file_name);

            if assembly_file_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {assembly_file_path:?} (use --overwrite to force).",
                );
            } else {
                File::create(&assembly_file_path)
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} creating error: {}", assembly_file_path, error)
                    })?
                    .write_all(assembly.as_bytes())
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} writing error: {}", assembly_file_path, error)
                    })?;
            }
        }

        let binary_file_name = format!(
            "{}.{}",
            contract_name,
            era_compiler_common::EXTENSION_ERAVM_BINARY
        );
        let mut binary_file_path = output_directory.to_owned();
        binary_file_path.push(binary_file_name);

        if binary_file_path.exists() && !overwrite {
            anyhow::bail!(
                "Refusing to overwrite an existing file {binary_file_path:?} (use --overwrite to force).",
            );
        } else {
            File::create(&binary_file_path)
                .map_err(|error| {
                    anyhow::anyhow!("File {:?} creating error: {}", binary_file_path, error)
                })?
                .write_all(format!("0x{}", hex::encode(self.build.bytecode.as_slice())).as_bytes())
                .map_err(|error| {
                    anyhow::anyhow!("File {:?} writing error: {}", binary_file_path, error)
                })?;
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        self,
        combined_json_contract: &mut CombinedJsonContract,
    ) -> anyhow::Result<()> {
        let hexadecimal_bytecode = hex::encode(self.build.bytecode);
        combined_json_contract.bytecode = Some(hexadecimal_bytecode);
        combined_json_contract
            .bytecode_runtime
            .clone_from(&combined_json_contract.bytecode);
        combined_json_contract.assembly = self.build.assembly;
        combined_json_contract.warnings = Some(self.warnings);
        combined_json_contract.factory_deps = Some(self.build.factory_dependencies);

        Ok(())
    }

    ///
    /// Extracts the contract file name from the full path.
    ///
    pub fn contract_name(path: &str) -> String {
        let path = path.trim().replace('\\', "/");
        path.split('/').last().expect("Always exists").to_owned()
    }
}
