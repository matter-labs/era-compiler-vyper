//!
//! The Vyper project.
//!

pub mod contract;
pub mod dependency_data;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use sha3::Digest;

use crate::build::contract::Contract as ContractBuild;
use crate::build::Build;
use crate::process::input::Input as ProcessInput;
use crate::process::output::Output as ProcessOutput;
use crate::warning_type::WarningType;

use self::contract::eravm_assembly::Contract as EraVMAssemblyContract;
use self::contract::llvm_ir::Contract as LLVMIRContract;
use self::contract::Contract;

///
/// The Vyper project.
///
#[derive(Debug, Clone)]
pub struct Project {
    /// The Vyper compiler version.
    pub version: semver::Version,
    /// The project source code hash.
    pub source_code_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
    /// The contract data,
    pub contracts: BTreeMap<String, Contract>,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        version: semver::Version,
        source_code_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
        contracts: BTreeMap<String, Contract>,
    ) -> Self {
        Self {
            version,
            source_code_hash,
            contracts,
        }
    }

    ///
    /// Parses the LLVM IR source code file and returns the source data.
    ///
    pub fn try_from_llvm_ir_path(path: &Path) -> anyhow::Result<Self> {
        let source_code = std::fs::read_to_string(path)
            .map_err(|error| anyhow::anyhow!("LLVM IR file {:?} reading error: {}", path, error))?;
        let path = path.to_string_lossy().to_string();

        let source_code_hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

        let mut project_contracts = BTreeMap::new();
        project_contracts.insert(
            path,
            LLVMIRContract::new(era_compiler_llvm_context::LLVM_VERSION, source_code).into(),
        );

        Ok(Self::new(
            era_compiler_llvm_context::LLVM_VERSION,
            source_code_hash,
            project_contracts,
        ))
    }

    ///
    /// Parses the EraVM assembly source code file and returns the source data.
    ///
    pub fn try_from_eravm_assembly_path(path: &Path) -> anyhow::Result<Self> {
        let source_code = std::fs::read_to_string(path).map_err(|error| {
            anyhow::anyhow!("EraVM assembly file {:?} reading error: {}", path, error)
        })?;
        let path = path.to_string_lossy().to_string();

        let source_code_hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

        let mut project_contracts = BTreeMap::new();
        project_contracts.insert(
            path,
            EraVMAssemblyContract::new(
                era_compiler_llvm_context::eravm_const::ZKEVM_VERSION,
                source_code,
            )
            .into(),
        );

        Ok(Self::new(
            era_compiler_llvm_context::eravm_const::ZKEVM_VERSION,
            source_code_hash,
            project_contracts,
        ))
    }

    ///
    /// Compiles all contracts, returning the build.
    ///
    pub fn compile(
        self,
        evm_version: Option<era_compiler_common::EVMVersion>,
        include_metadata_hash: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        bytecode_encoding: zkevm_assembly::RunningVmEncodingMode,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Build> {
        let mut build = Build::default();
        let source_code_hash = if include_metadata_hash {
            Some(self.source_code_hash)
        } else {
            None
        };
        let results: BTreeMap<String, anyhow::Result<ContractBuild>> = self
            .contracts
            .par_iter()
            .map(|(full_path, contract)| {
                let process_output: anyhow::Result<ProcessOutput> =
                    crate::process::call(ProcessInput::new(
                        Cow::Borrowed(full_path),
                        Cow::Borrowed(contract),
                        source_code_hash,
                        bytecode_encoding == zkevm_assembly::RunningVmEncodingMode::Testing,
                        evm_version,
                        optimizer_settings.clone(),
                        llvm_options.clone(),
                        output_assembly,
                        suppressed_warnings.clone(),
                        debug_config.clone(),
                    ));

                (
                    full_path.to_owned(),
                    process_output.map(|output| output.build),
                )
            })
            .collect();

        let is_minimal_proxy_used = results.iter().any(|(_path, result)| {
            result
                .as_ref()
                .map(|contract| {
                    contract
                        .build
                        .factory_dependencies
                        .contains_key(crate::r#const::FORWARDER_CONTRACT_HASH.as_str())
                })
                .unwrap_or_default()
        });
        if is_minimal_proxy_used {
            let minimal_proxy_build = era_compiler_llvm_context::EraVMBuild::new(
                crate::r#const::FORWARDER_CONTRACT_BYTECODE.clone(),
                crate::r#const::FORWARDER_CONTRACT_HASH.clone(),
                None,
                if output_assembly {
                    Some(crate::r#const::FORWARDER_CONTRACT_ASSEMBLY.to_owned())
                } else {
                    None
                },
            );
            build.contracts.insert(
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
                ContractBuild::new(minimal_proxy_build, vec![]),
            );
        }

        let mut errors = Vec::with_capacity(results.len());
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    build.contracts.insert(path, contract);
                }
                Err(error) => {
                    errors.push((path, error));
                }
            }
        }

        if !errors.is_empty() {
            anyhow::bail!(
                "{}",
                errors
                    .into_iter()
                    .map(|(path, error)| format!("Contract `{path}`: {error}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }

        Ok(build)
    }
}
