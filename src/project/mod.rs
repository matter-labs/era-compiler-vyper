//!
//! The Vyper project.
//!

pub mod contract;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::build::contract::Contract as ContractBuild;
use crate::build::Build;
use crate::process::input::Input as ProcessInput;
use crate::process::output::Output as ProcessOutput;
use crate::project::contract::vyper::ast::AST as VyperAST;
use crate::project::contract::vyper::Contract as VyperContract;
use crate::project::contract::Contract as ProjectContract;
use crate::vyper::selector::Selector as VyperSelector;
use crate::vyper::standard_json::output::Output as VyperStandardJsonOutput;
use crate::warning_type::WarningType;

use self::contract::eravm_assembly::Contract as EraVMAssemblyContract;
use self::contract::llvm_ir::Contract as LLVMIRContract;
use self::contract::metadata::Metadata as ContractMetadata;
use self::contract::Contract;

///
/// The Vyper project.
///
#[derive(Debug, Clone)]
pub struct Project {
    /// The Vyper compiler version.
    pub version: semver::Version,
    /// The contract data,
    pub contracts: BTreeMap<String, Contract>,
    /// The selection output.
    pub output_selection: Vec<VyperSelector>,
    /// The project source code hash.
    pub project_hash: era_compiler_common::Hash,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        version: semver::Version,
        contracts: BTreeMap<String, Contract>,
        output_selection: Vec<VyperSelector>,
    ) -> Self {
        let source_codes = contracts
            .values()
            .map(|contract| contract.source_code().as_bytes())
            .collect::<Vec<&[u8]>>();
        let project_hash = era_compiler_common::Hash::keccak256_multiple(source_codes.as_slice());

        Self {
            version,
            contracts,
            output_selection,
            project_hash,
        }
    }

    ///
    /// Converts Vyper standard JSON output into a project.
    ///
    pub fn try_from_standard_json(
        standard_json: VyperStandardJsonOutput,
        version: &semver::Version,
    ) -> anyhow::Result<Self> {
        let files = standard_json.contracts.unwrap_or_default();
        let errors = standard_json.errors.unwrap_or_default();
        if files.is_empty() && !errors.is_empty() {
            anyhow::bail!(
                "{}",
                errors
                    .into_iter()
                    .map(|error| error.message)
                    .collect::<Vec<String>>()
                    .join("\n\n")
            );
        }

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
                    contract.ir,
                    ast,
                    serde_json::Value::Null,
                    contract.evm.method_identifiers,
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
        output_selection: &[VyperSelector],
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
            output_selection.to_owned(),
        ))
    }

    ///
    /// Reads EraVM assembly source code files and returns the project.
    ///
    pub fn try_from_eravm_assembly_paths(
        paths: &[&Path],
        output_selection: &[VyperSelector],
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
            output_selection.to_owned(),
        ))
    }

    ///
    /// Compiles all contracts, returning the build.
    ///
    pub fn compile(
        self,
        evm_version: Option<era_compiler_common::EVMVersion>,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Build> {
        let metadata = ContractMetadata::new(
            self.project_hash.as_bytes(),
            &self.version,
            evm_version,
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer_settings.clone(),
            llvm_options.as_slice(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_bytes = metadata_json.to_string().into_bytes();
        let metadata_hash = match metadata_hash_type {
            era_compiler_common::HashType::None => None,
            era_compiler_common::HashType::Keccak256 => Some(era_compiler_common::Hash::keccak256(
                metadata_bytes.as_slice(),
            )),
            era_compiler_common::HashType::Ipfs => {
                Some(era_compiler_common::Hash::ipfs(metadata_bytes.as_slice()))
            }
        };

        let mut build = Build::new(metadata_json);
        let results: BTreeMap<String, anyhow::Result<ContractBuild>> = self
            .contracts
            .par_iter()
            .map(|(full_path, contract)| {
                let process_output: anyhow::Result<ProcessOutput> = crate::process::call(
                    full_path.as_str(),
                    ProcessInput::new(
                        Cow::Borrowed(full_path),
                        Cow::Borrowed(contract),
                        metadata_hash.clone(),
                        self.output_selection.clone(),
                        optimizer_settings.clone(),
                        llvm_options.clone(),
                        suppressed_warnings.clone(),
                        debug_config.clone(),
                    ),
                );

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
                    contract.build.factory_dependencies.contains_key(
                        hex::encode(
                            crate::r#const::MINIMAL_PROXY_BUILD
                                .bytecode_hash
                                .expect("Always exists")
                                .as_slice(),
                        )
                        .as_str(),
                    )
                })
                .unwrap_or_default()
        });
        if is_minimal_proxy_used {
            build.contracts.insert(
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
                ContractBuild::new_minimal_proxy(
                    self.output_selection
                        .contains(&VyperSelector::EraVMAssembly),
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
