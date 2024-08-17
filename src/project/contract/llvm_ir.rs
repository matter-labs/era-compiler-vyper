//!
//! The LLVM IR contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::message_type::MessageType;
use crate::project::contract::metadata::Metadata as ContractMetadata;
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
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_selection: Vec<VyperSelection>,
        _suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings.clone());

        let metadata = ContractMetadata::new(
            self.source_code.as_str(),
            &self.version,
            None,
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer_settings,
            llvm_options.as_slice(),
        );
        let metadata_bytes = serde_json::to_vec(&metadata).expect("Always valid");
        let metadata_hash = match metadata_hash_type {
            era_compiler_common::HashType::None => None,
            era_compiler_common::HashType::Keccak256 => Some(era_compiler_common::Hash::keccak256(
                metadata_bytes.as_slice(),
            )),
            era_compiler_common::HashType::Ipfs => {
                Some(era_compiler_common::Hash::ipfs(metadata_bytes.as_slice()))
            }
        };

        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
            self.source_code.as_bytes(),
            contract_path,
        );
        let module = llvm
            .create_module_from_ir(memory_buffer)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?;
        let context = era_compiler_llvm_context::EraVMContext::<
            era_compiler_llvm_context::DummyDependency,
        >::new(&llvm, module, llvm_options, optimizer, None, debug_config);

        let build = context.build(
            contract_path,
            metadata_hash,
            output_selection.contains(&VyperSelection::EraVMAssembly),
            false,
        )?;

        Ok(ContractBuild::new_inner(build))
    }
}
