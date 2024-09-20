//!
//! The `vyper --standard-json` output contract EVM data.
//!

use std::collections::BTreeMap;

///
/// The `vyper --standard-json` output contract EVM data.
///
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    // /// The contract ABI.
    // pub abi: serde_json::Value,
    /// The contract method identifiers.
    pub method_identifiers: BTreeMap<String, String>,
}
