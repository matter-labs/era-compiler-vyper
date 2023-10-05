//!
//! The Vyper contract metadata.
//!

use serde::Serialize;
use sha3::Digest;

///
/// The Vyper contract metadata.
///
/// Is used to append the metadata hash to the contract bytecode.
///
#[derive(Debug, Serialize)]
pub struct Metadata<'a> {
    /// The source code hash.
    pub source_hash: &'a [u8; compiler_common::BYTE_LENGTH_FIELD],
    /// The source file upstream Vyper compiler version.
    pub source_version: &'a semver::Version,
    /// The EraVM compiler version.
    pub zk_version: semver::Version,
    /// The EraVM compiler stringified optimizer settings.
    pub optimizer_settings: String,
}

impl<'a> Metadata<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        source_hash: &'a [u8; compiler_common::BYTE_LENGTH_FIELD],
        source_version: &'a semver::Version,
        zk_version: semver::Version,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
    ) -> Self {
        Self {
            source_hash,
            source_version,
            zk_version,
            optimizer_settings: optimizer_settings.to_string(),
        }
    }

    ///
    /// Returns the `keccak256` hash of the metadata.
    ///
    pub fn keccak256(&self) -> [u8; compiler_common::BYTE_LENGTH_FIELD] {
        let json = serde_json::to_vec(self).expect("Always valid");
        let hash = sha3::Keccak256::digest(json.as_slice());
        hash.into()
    }
}
