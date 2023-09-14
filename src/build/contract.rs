//!
//! The Vyper contract build.
//!

use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::vyper::combined_json::contract::Contract as CombinedJsonContract;

///
/// The Vyper contract build.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    /// The LLVM module build.
    pub build: compiler_llvm_context::EraVMBuild,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: compiler_llvm_context::EraVMBuild) -> Self {
        Self { build }
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
        let contract_name = Self::short_path(contract_path.to_str().expect("Always valid"));

        let assembly_file_name = format!(
            "{}.{}",
            contract_name,
            compiler_common::EXTENSION_ERAVM_ASSEMBLY
        );
        let mut assembly_file_path = output_directory.to_owned();
        assembly_file_path.push(assembly_file_name);

        if assembly_file_path.exists() && !overwrite {
            eprintln!(
                "Refusing to overwrite an existing file {assembly_file_path:?} (use --overwrite to force).",
            );
        } else {
            File::create(&assembly_file_path)
                .map_err(|error| {
                    anyhow::anyhow!("File {:?} creating error: {}", assembly_file_path, error)
                })?
                .write_all(self.build.assembly_text.as_bytes())
                .map_err(|error| {
                    anyhow::anyhow!("File {:?} writing error: {}", assembly_file_path, error)
                })?;
        }

        let binary_file_name = format!(
            "{}.{}",
            contract_name,
            compiler_common::EXTENSION_ERAVM_BINARY
        );
        let mut binary_file_path = output_directory.to_owned();
        binary_file_path.push(binary_file_name);

        if binary_file_path.exists() && !overwrite {
            eprintln!(
                "Refusing to overwrite an existing file {binary_file_path:?} (use --overwrite to force).",
            );
        } else {
            File::create(&binary_file_path)
                .map_err(|error| {
                    anyhow::anyhow!("File {:?} creating error: {}", binary_file_path, error)
                })?
                .write_all(self.build.bytecode.as_slice())
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
        match (
            combined_json_contract.bytecode.as_mut(),
            combined_json_contract.bytecode_runtime.as_mut(),
        ) {
            (Some(bytecode), Some(bytecode_runtime)) => {
                *bytecode = hexadecimal_bytecode;
                *bytecode_runtime = bytecode.clone();
            }
            (Some(bytecode), None) => {
                *bytecode = hexadecimal_bytecode;
            }
            (None, Some(bytecode_runtime)) => {
                *bytecode_runtime = hexadecimal_bytecode;
            }
            (None, None) => {}
        }

        combined_json_contract.factory_deps = Some(self.build.factory_dependencies);

        Ok(())
    }

    ///
    /// Converts the full path to a short one.
    ///
    pub fn short_path(path: &str) -> &str {
        path.rfind('/')
            .map(|last_slash| &path[last_slash + 1..])
            .unwrap_or_else(|| path)
    }
}
