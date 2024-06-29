//!
//! The EraVM assembly contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::message_type::MessageType;
use crate::project::contract::metadata::Metadata as ContractMetadata;

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
        source_code_hash: Option<[u8; era_compiler_common::BYTE_LENGTH_FIELD]>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        _suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let metadata_hash = source_code_hash.map(|source_code_hash| {
            ContractMetadata::new(
                &source_code_hash,
                &self.version,
                None,
                semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
                optimizer_settings,
                llvm_options.as_slice(),
            )
            .keccak256()
        });

        let build = era_compiler_llvm_context::from_eravm_assembly(
            contract_path,
            self.source_code,
            metadata_hash,
            output_assembly,
            debug_config.as_ref(),
        )?;

        Ok(ContractBuild::new(build, vec![]))
    }
}
