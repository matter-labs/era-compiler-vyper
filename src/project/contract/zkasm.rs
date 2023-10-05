//!
//! The EraVM assembly contract.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::build::contract::Contract as ContractBuild;
use crate::project::contract::metadata::Metadata as ContractMetadata;
use crate::warning_type::WarningType;

///
/// The EraVM assembly contract.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
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
        source_code_hash: Option<[u8; compiler_common::BYTE_LENGTH_FIELD]>,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        _suppressed_warnings: Vec<WarningType>,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let metadata_hash = source_code_hash.map(|source_code_hash| {
            ContractMetadata::new(
                &source_code_hash,
                &self.version,
                semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
                optimizer_settings,
            )
            .keccak256()
        });

        let build = compiler_llvm_context::eravm_build_assembly_text(
            contract_path,
            self.source_code.as_str(),
            metadata_hash,
            debug_config.as_ref(),
        )?;

        Ok(ContractBuild::new(build, vec![]))
    }
}
