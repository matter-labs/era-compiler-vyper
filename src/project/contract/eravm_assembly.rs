//!
//! The EraVM assembly contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::message_type::MessageType;
use crate::project::contract::metadata::Metadata as ContractMetadata;
use crate::vyper::selection::Selection as VyperSelection;

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
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        _output_selection: Vec<VyperSelection>,
        _suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let target_machine = era_compiler_llvm_context::TargetMachine::new(
            era_compiler_common::Target::EraVM,
            &optimizer_settings,
            llvm_options.as_slice(),
        )?;

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
