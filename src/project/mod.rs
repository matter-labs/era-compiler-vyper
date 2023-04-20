//!
//! The Vyper project representation.
//!

pub mod contract;
pub mod dependency_data;

use std::collections::BTreeMap;
use std::path::Path;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use sha3::Digest;

use crate::build::contract::Contract as ContractBuild;
use crate::build::Build;

use self::contract::llvm_ir::Contract as LLVMIRContract;
use self::contract::Contract;

///
/// The Vyper project representation.
///
#[derive(Debug, Clone)]
pub struct Project {
    /// The Vyper compiler version.
    pub version: semver::Version,
    /// The project source code hash.
    pub source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD],
    /// The contract data,
    pub contracts: BTreeMap<String, Contract>,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        version: semver::Version,
        source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD],
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
        project_contracts.insert(path, LLVMIRContract::new(source_code).into());

        Ok(Self::new(
            compiler_llvm_context::LLVM_VERSION,
            source_code_hash,
            project_contracts,
        ))
    }

    ///
    /// Compiles all contracts, returning the build.
    ///
    pub fn compile(
        self,
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        include_metadata_hash: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Build> {
        let mut build = Build::default();
        let results: BTreeMap<String, anyhow::Result<ContractBuild>> = self
            .contracts
            .into_par_iter()
            .map(|(path, contract)| {
                let contract_build = contract.compile(
                    path.as_str(),
                    self.source_code_hash,
                    target_machine.clone(),
                    optimizer_settings.clone(),
                    include_metadata_hash,
                    debug_config.clone(),
                );
                (path, contract_build)
            })
            .collect();

        let is_forwarder_used = results.iter().any(|(_path, result)| {
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
        if is_forwarder_used {
            let forwarder_build = compiler_llvm_context::Build::new(
                crate::r#const::FORWARDER_CONTRACT_ASSEMBLY.to_owned(),
                zkevm_assembly::Assembly::from_string(
                    crate::r#const::FORWARDER_CONTRACT_ASSEMBLY.to_owned(),
                    Some(
                        sha3::Keccak256::digest(crate::r#const::FORWARDER_CONTRACT_ASSEMBLY).into(),
                    ),
                )?,
                crate::r#const::FORWARDER_CONTRACT_BYTECODE.clone(),
                crate::r#const::FORWARDER_CONTRACT_HASH.clone(),
            );
            build.contracts.insert(
                crate::r#const::FORWARDER_CONTRACT_NAME.to_owned(),
                ContractBuild::new(forwarder_build),
            );
        }

        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    build.contracts.insert(path, contract);
                }
                Err(error) => {
                    anyhow::bail!("Contract `{}` compiling error: {:?}", path, error);
                }
            }
        }

        Ok(build)
    }
}
