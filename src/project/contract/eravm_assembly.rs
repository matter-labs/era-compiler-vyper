//!
//! The EraVM assembly contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::vyper::selector::Selector as VyperSelector;
use crate::warning_type::WarningType;

///
/// The EraVM assembly contract.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Contract {
    /// The contract source code.
    pub source_code: String,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(source_code: String) -> Self {
        Self { source_code }
    }

    ///
    /// Compiles the contract, returning the build.
    ///
    pub fn compile(
        self,
        contract_path: &str,
        metadata_hash: Option<era_compiler_common::Hash>,
        no_bytecode_metadata: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        _output_selection: Vec<VyperSelector>,
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

        let cbor_data = if no_bytecode_metadata {
            None
        } else {
            let cbor_key = crate::r#const::VYPER_PRODUCTION_NAME.to_owned();
            let cbor_data = vec![(
                crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
                crate::r#const::version().parse().expect("Always valid"),
            )];
            Some((cbor_key, cbor_data))
        };

        let build = era_compiler_llvm_context::eravm_build(
            bytecode_buffer,
            metadata_hash,
            cbor_data,
            Some(self.source_code),
        )?;

        Ok(ContractBuild::new_inner(build))
    }
}
