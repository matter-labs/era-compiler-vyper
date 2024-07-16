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
use sha3::digest::FixedOutput;
use sha3::Digest;

use crate::build::contract::Contract as ContractBuild;
use crate::build::Build;
use crate::message_type::MessageType;
use crate::metadata::Metadata as SourceMetadata;
use crate::process::input::Input as ProcessInput;
use crate::process::output::Output as ProcessOutput;
use crate::project::contract::vyper::ast::AST as VyperAST;
use crate::project::contract::vyper::Contract as VyperContract;
use crate::project::contract::Contract as ProjectContract;
use crate::vyper::selection::Selection as VyperSelection;
use crate::vyper::standard_json::output::Output as VyperStandardJsonOutput;

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
    /// The selection output.
    pub output_selection: Vec<VyperSelection>,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        version: semver::Version,
        contracts: BTreeMap<String, Contract>,
        output_selection: Vec<VyperSelection>,
    ) -> Self {
        let mut source_code_hasher = sha3::Keccak256::new();
        for (_path, contract) in contracts.iter() {
            source_code_hasher.update(contract.source_code().as_bytes());
        }
        let source_code_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD] =
            source_code_hasher.finalize_fixed().into();

        Self {
            version,
            source_code_hash,
            contracts,
            output_selection,
        }
    }

    ///
    /// Converts Vyper standard JSON output into a project.
    ///
    pub fn try_from_standard_json(
        mut standard_json: VyperStandardJsonOutput,
        version: &semver::Version,
    ) -> anyhow::Result<Self> {
        let files = match standard_json.contracts.take() {
            Some(files) => files,
            None => {
                anyhow::bail!(
                    "{}",
                    standard_json
                        .errors
                        .as_ref()
                        .map(|errors| serde_json::to_string_pretty(errors).expect("Always valid"))
                        .unwrap_or_else(|| "Unknown project assembling error".to_owned())
                );
            }
        };

        let mut project_contracts: BTreeMap<String, ProjectContract> = BTreeMap::new();
        for (path, file) in files.into_iter() {
            for (name, contract) in file.into_iter() {
                let full_path = format!("{path}:{name}");

                let ast = standard_json
                    .sources
                    .as_ref()
                    .and_then(|sources| sources.get(&path).cloned())
                    .ok_or_else(|| anyhow::anyhow!("No AST for contract {}", full_path))?;
                let ast = VyperAST::new(full_path.clone(), ast);

                let project_contract = VyperContract::new(
                    version.to_owned(),
                    contract.source_code.expect("Must be set by the tester"),
                    String::new(),
                    contract.ir,
                    SourceMetadata::default(),
                    ast,
                    serde_json::Value::Null,
                    contract.evm.method_identifiers,
                    None,
                    None,
                    None,
                    None,
                    None,
                );
                project_contracts.insert(full_path, project_contract.into());
            }
        }

        Ok(Self::new(version.to_owned(), project_contracts, vec![]))
    }

    ///
    /// Reads LLVM IR source code files and returns the project.
    ///
    pub fn try_from_llvm_ir_paths(
        paths: &[&Path],
        output_selection: Vec<VyperSelection>,
    ) -> anyhow::Result<Self> {
        let contracts = paths
            .iter()
            .map(|path| {
                let source_code = std::fs::read_to_string(path).map_err(|error| {
                    anyhow::anyhow!("LLVM IR file {path:?} reading error: {error}")
                })?;
                let path = path.to_string_lossy().to_string();

                let contract =
                    LLVMIRContract::new(era_compiler_llvm_context::LLVM_VERSION, source_code)
                        .into();

                Ok((path, contract))
            })
            .collect::<anyhow::Result<BTreeMap<String, Contract>>>()?;

        Ok(Self::new(
            era_compiler_llvm_context::LLVM_VERSION,
            contracts,
            output_selection,
        ))
    }

    ///
    /// Reads EraVM assembly source code files and returns the project.
    ///
    pub fn try_from_eravm_assembly_paths(
        paths: &[&Path],
        output_selection: Vec<VyperSelection>,
    ) -> anyhow::Result<Self> {
        let contracts = paths
            .iter()
            .map(|path| {
                let source_code = std::fs::read_to_string(path).map_err(|error| {
                    anyhow::anyhow!("EraVM assembly file {path:?} reading error: {error}")
                })?;
                let path = path.to_string_lossy().to_string();

                let contract = EraVMAssemblyContract::new(
                    era_compiler_llvm_context::eravm_const::ZKEVM_VERSION,
                    source_code,
                )
                .into();

                Ok((path, contract))
            })
            .collect::<anyhow::Result<BTreeMap<String, Contract>>>()?;

        Ok(Self::new(
            era_compiler_llvm_context::eravm_const::ZKEVM_VERSION,
            contracts,
            output_selection,
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
        bytecode_encoding: zkevm_assembly::RunningVmEncodingMode,
        suppressed_messages: Vec<MessageType>,
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
                        self.output_selection.clone(),
                        optimizer_settings.clone(),
                        llvm_options.clone(),
                        suppressed_messages.clone(),
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
                        .contains_key(crate::r#const::MINIMAL_PROXY_CONTRACT_HASH.as_str())
                })
                .unwrap_or_default()
        });
        if is_minimal_proxy_used {
            build.contracts.insert(
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
                ContractBuild::new_minimal_proxy(
                    self.output_selection
                        .contains(&VyperSelection::EraVMAssembly),
                ),
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
