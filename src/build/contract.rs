//!
//! The Vyper contract build.
//!

use std::collections::BTreeMap;
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
    /// The `vyper` method identifiers output.
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The `vyper` ABI output.
    pub abi: Option<serde_json::Value>,
    /// The compilation warnings.
    pub warnings: Vec<CombinedJsonContractWarning>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        build: era_compiler_llvm_context::EraVMBuild,
        method_identifiers: Option<BTreeMap<String, String>>,
        abi: Option<serde_json::Value>,
        warnings: Vec<CombinedJsonContractWarning>,
    ) -> Self {
        Self {
            build,
            method_identifiers,
            abi,
            warnings,
        }
    }

    ///
    /// A shortcut constructor for minimal proxy.
    ///
    pub fn new_minimal_proxy(build: era_compiler_llvm_context::EraVMBuild) -> Self {
        Self {
            build,
            method_identifiers: None,
            abi: None,
            warnings: vec![],
        }
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
        let contract_path = crate::path_to_posix(contract_path)?;
        let file_name = contract_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("File name not found in path {contract_path:?}"))?
            .to_string_lossy();

        if let Some(assembly) = self.build.assembly {
            let assembly_file_name = format!(
                "{}.{}",
                file_name,
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
            file_name,
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
}

impl From<Contract> for CombinedJsonContract {
    fn from(contract: Contract) -> Self {
        Self {
            method_identifiers: contract.method_identifiers,
            abi: contract.abi,

            bytecode: Some(hex::encode(contract.build.bytecode)),
            assembly: contract.build.assembly,
            warnings: Some(contract.warnings),
            factory_deps: Some(contract.build.factory_dependencies),
        }
    }
}
