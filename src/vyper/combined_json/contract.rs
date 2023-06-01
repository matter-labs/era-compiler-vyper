//!
//! The `vyper --combined-json` contract.
//!

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

///
/// The contract.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    /// The `vyper` method identifiers output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The `vyper` ABI output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,
    /// The `vyper` hexadecimal binary output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<String>,
    /// The `vyper` hexadecimal binary runtime part output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode_runtime: Option<String>,
    /// The factory dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_deps: Option<BTreeMap<String, String>>,
}

impl Contract {
    ///
    /// Creates a forwarder.
    ///
    pub fn new_forwarder() -> Self {
        Self {
            method_identifiers: Some(BTreeMap::new()),
            abi: Some(serde_json::Value::Object(serde_json::Map::default())),
            bytecode: Some(hex::encode(
                crate::r#const::FORWARDER_CONTRACT_BYTECODE.as_slice(),
            )),
            bytecode_runtime: Some(hex::encode(
                crate::r#const::FORWARDER_CONTRACT_BYTECODE.as_slice(),
            )),
            factory_deps: Some(BTreeMap::new()),
        }
    }

    ///
    /// Returns the signature hash of the specified contract entry.
    ///
    /// # Panics
    /// If the hashes have not been requested in the `vyper` call.
    ///
    pub fn entry(&self, entry: &str) -> u32 {
        self.method_identifiers
            .as_ref()
            .expect("Always exists")
            .iter()
            .find_map(|(contract_entry, hash)| {
                if contract_entry.starts_with(&(entry.to_owned() + "(")) {
                    Some(
                        u32::from_str_radix(hash.as_str(), compiler_common::BASE_HEXADECIMAL)
                            .expect("Test hash is always valid"),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Entry `{entry}` not found"))
    }
}
