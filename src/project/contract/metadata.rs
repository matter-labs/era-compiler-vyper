//!
//! The Vyper contract metadata.
//!

///
/// The Vyper contract metadata.
///
/// Is used to append the metadata hash to the contract bytecode.
///
#[derive(Debug, serde::Serialize)]
pub struct Metadata<'a> {
    /// The source code hash.
    pub source_code_hash: &'a [u8],
    /// The source file upstream Vyper compiler version.
    pub source_version: &'a semver::Version,
    /// The EVM target version.
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// The EraVM compiler version.
    pub zk_version: semver::Version,
    /// The EraVM compiler stringified optimizer settings.
    pub optimizer_settings: String,
    /// The LLVM extra arguments.
    pub llvm_options: &'a [String],
}

impl<'a> Metadata<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        source_code_hash: &'a [u8],
        source_version: &'a semver::Version,
        evm_version: Option<era_compiler_common::EVMVersion>,
        zk_version: semver::Version,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &'a [String],
    ) -> Self {
        Self {
            source_code_hash,
            source_version,
            evm_version,
            zk_version,
            optimizer_settings: optimizer_settings.to_string(),
            llvm_options,
        }
    }
}
