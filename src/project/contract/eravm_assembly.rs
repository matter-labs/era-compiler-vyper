//!
//! The EraVM assembly contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::vyper::selection::Selection as VyperSelection;
use crate::warning_type::WarningType;

///
/// The EraVM assembly contract.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Contract {
    /// The EraVM version.
    pub version: semver::Version,
    /// The contract source code.
    pub source_code: String,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(version: semver::Version, source_code: String) -> Self {
        Self {
            version,
            source_code,
        }
    }

    ///
    /// Compiles the contract, returning the build.
    ///
    pub fn compile(
        self,
        contract_path: &str,
        metadata_hash: Option<era_compiler_common::Hash>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        _output_selection: Vec<VyperSelection>,
        _suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let target_machine = era_compiler_llvm_context::TargetMachine::new(
            era_compiler_common::Target::EraVM,
            &optimizer_settings,
            llvm_options.as_slice(),
        )?;

        let bytecode_buffer = era_compiler_llvm_context::eravm_assemble(
            &target_machine,
            contract_path,
            self.source_code.as_str(),
            debug_config.as_ref(),
        )?;

        let build = era_compiler_llvm_context::eravm_build(
            bytecode_buffer,
            metadata_hash,
            Some(self.source_code),
        )?;

        Ok(ContractBuild::new_inner(build))
    }
}
