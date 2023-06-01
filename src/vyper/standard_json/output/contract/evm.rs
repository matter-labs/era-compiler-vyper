//!
//! The `vyper --standard-json` output contract EVM data.
//!

use std::collections::BTreeMap;

use serde::Deserialize;

///
/// The `vyper --standard-json` output contract EVM data.
///
#[derive(Debug, Deserialize)]
pub struct EVM {
    /// The contract ABI.
    #[serde(rename = "methodIdentifiers")]
    pub abi: BTreeMap<String, String>,
}
