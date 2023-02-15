//!
//! The LLVM IR contract representation.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::project::dependency_data::DependencyData;

///
/// The LLVM IR contract representation.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The source code.
    pub code: String,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(code: String) -> Self {
        Self { code }
    }

    ///
    /// Compiles the contract, returning the build.
    ///
    pub fn compile(
        self,
        contract_path: &str,
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let llvm = inkwell::context::Context::create();
        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
            self.code.as_bytes(),
            contract_path,
        );
        let module = llvm
            .create_module_from_ir(memory_buffer)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        let optimizer = compiler_llvm_context::Optimizer::new(target_machine, optimizer_settings);
        let context = compiler_llvm_context::Context::<DependencyData>::new(
            &llvm,
            module,
            optimizer,
            None,
            debug_config,
        );
        let build = context.build(contract_path)?;

        Ok(ContractBuild::new(build))
    }
}
