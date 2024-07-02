//!
//! The `vyper --standard-json` output contract EVM data.
//!

use std::collections::BTreeMap;

///
/// The `vyper --standard-json` output contract EVM data.
///
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
pub struct EVM {
    /// The contract ABI.
    pub method_identifiers: BTreeMap<String, String>,
}
