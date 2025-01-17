//!
//! The Vyper project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use crate::vyper::combined_json::extra_data::ExtraData as CombinedJsonExtraData;
use crate::vyper::combined_json::CombinedJson;
use crate::vyper::selector::Selector as VyperSelector;
use crate::vyper::Compiler as VyperCompiler;

use self::contract::Contract;

///
/// The Vyper project build.
///
#[derive(Debug)]
pub struct Build {
    /// The contract data,
    pub contracts: BTreeMap<String, Contract>,
    /// The project metadata.
    pub project_metadata: serde_json::Value,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(project_metadata: serde_json::Value) -> Self {
        Self {
            contracts: BTreeMap::new(),
            project_metadata,
        }
    }

    ///
    /// Links the EraVM build.
    ///
    pub fn link(
        &mut self,
        linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
    ) -> anyhow::Result<()> {
        let mut factory_dependencies = BTreeMap::new();
        factory_dependencies.insert(
            crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
            crate::r#const::MINIMAL_PROXY_BUILD
                .bytecode_hash
                .expect("Always exists"),
        );

        for (path, contract) in self.contracts.iter_mut() {
            let bytecode_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                contract.build.bytecode.as_slice(),
                path.as_str(),
                false,
            );
            let (bytecode_buffer_linked, object_format) = era_compiler_llvm_context::eravm_link(
                bytecode_buffer,
                &linker_symbols,
                &factory_dependencies,
            )?;
            assert_eq!(
                object_format,
                era_compiler_common::ObjectFormat::Raw,
                "Linked Vyper bytecode cannot be ELF"
            );

            // Reassign bytecode only after getting the hash, since `bytecode_buffer_linked` and `contract.build.bytecode` can refer to the same memory segment.
            contract.build.bytecode_hash = Some(era_compiler_llvm_context::eravm_hash(
                &bytecode_buffer_linked,
            )?);
            contract.build.bytecode = bytecode_buffer_linked.as_slice().to_vec();
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(self, selection: &[VyperSelector]) -> anyhow::Result<()> {
        for (path, contract) in self.contracts.into_iter() {
            contract.write_to_terminal(path, selection)?;
        }

        if selection.contains(&VyperSelector::ProjectMetadata) {
            writeln!(std::io::stderr(), "Project metadata:")?;
            writeln!(std::io::stdout(), "{}", self.project_metadata)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        selection: &[VyperSelector],
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

        if selection.contains(&VyperSelector::ProjectMetadata) {
            let metadata_file_name = format!("meta.{}", era_compiler_common::EXTENSION_JSON);
            let mut metadata_file_path = output_directory.to_owned();
            metadata_file_path.push(metadata_file_name);
            if metadata_file_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {metadata_file_path:?} (use --overwrite to force).",
                );
            }
            std::fs::write(
                &metadata_file_path,
                serde_json::to_string(&self.project_metadata)
                    .expect("Always valid")
                    .as_bytes(),
            )
            .map_err(|error| {
                anyhow::anyhow!("File {metadata_file_path:?} writing error: {error}")
            })?;
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

        let extra_data = CombinedJsonExtraData::new(self.project_metadata);

        CombinedJson::new(contracts, extra_data, version, zkvyper_version)
    }
}
