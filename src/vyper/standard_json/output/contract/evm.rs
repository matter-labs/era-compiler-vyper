//!
//! The `vyper --standard-json` output contract EVM data.
//!

use std::collections::BTreeMap;

///
/// The `vyper --standard-json` output contract EVM data.
///
#[derive(Debug, serde::Deserialize)]
pub struct EVM {
    /// The contract ABI.
    #[serde(rename = "methodIdentifiers")]
    pub abi: BTreeMap<String, String>,
}
