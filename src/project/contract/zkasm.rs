//!
//! The zkEVM assembly contract.
//!

use crate::build::contract::Contract as ContractBuild;
use crate::project::contract::metadata::Metadata as ContractMetadata;

///
/// The zkEVM assembly contract.
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
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        include_metadata_hash: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let metadata_hash = if include_metadata_hash {
            Some(
                ContractMetadata::new(
                    &source_code_hash,
                    &compiler_llvm_context::ZKEVM_VERSION,
                    semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
                    optimizer_settings,
                )
                .keccak256(),
            )
        } else {
            None
        };

        let build = compiler_llvm_context::build_assembly_text(
            contract_path,
            self.source_code.as_str(),
            metadata_hash,
            debug_config.as_ref(),
        )?;

        Ok(ContractBuild::new(build))
    }
}
