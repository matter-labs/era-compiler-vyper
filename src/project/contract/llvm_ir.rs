//!
//! The LLVM IR contract representation.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::project::contract::metadata::Metadata as ContractMetadata;
use crate::project::dependency_data::DependencyData;

///
/// The LLVM IR contract representation.
///
#[derive(Debug, Clone)]
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
        source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD],
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let llvm = inkwell::context::Context::create();
        let optimizer = compiler_llvm_context::Optimizer::new(target_machine, optimizer_settings);

        let metadata_hash = ContractMetadata::new(
            &source_code_hash,
            &compiler_llvm_context::LLVM_VERSION,
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer.settings().to_owned(),
        )
        .keccak256();

        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
            self.source_code.as_bytes(),
            contract_path,
        );
        let module = llvm
            .create_module_from_ir(memory_buffer)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        let context = compiler_llvm_context::Context::<DependencyData>::new(
            &llvm,
            module,
            optimizer,
            None,
            debug_config,
        );

        let build = context.build(contract_path, metadata_hash)?;

        Ok(ContractBuild::new(build))
    }
}
