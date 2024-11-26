//!
//! The LLVM IR contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::message_type::MessageType;
use crate::vyper::selection::Selection as VyperSelection;

///
/// The LLVM IR contract.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The LLVM framework version.
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
        output_selection: Vec<VyperSelection>,
        _suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings.clone());

        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
            self.source_code.as_bytes(),
            contract_path,
        );
        let module = llvm
            .create_module_from_ir(memory_buffer)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        let context = era_compiler_llvm_context::EraVMContext::<
            era_compiler_llvm_context::DummyDependency,
        >::new(&llvm, module, llvm_options, optimizer, debug_config);

        let build = context.build(
            contract_path,
            metadata_hash,
            output_selection.contains(&VyperSelection::EraVMAssembly),
            false,
        )?;

        Ok(ContractBuild::new_inner(build))
    }
}
