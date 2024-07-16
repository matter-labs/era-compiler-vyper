//!
//! The `vyper --combined-json` contract.
//!

pub mod warning;

use std::collections::BTreeMap;

use self::warning::Warning;

///
/// The contract.
///
#[derive(Debug, serde::Serialize)]
pub struct Contract {
    /// The `vyper` hexadecimal binary output.
    pub bytecode: String,

    /// The `vyper` method identifiers output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The `vyper` ABI output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,
    /// The `vyper` layout output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<serde_json::Value>,
    /// The `vyper` userdoc output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdoc: Option<serde_json::Value>,
    /// The `vyper` devdoc output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devdoc: Option<serde_json::Value>,

    /// The EraVM text assembly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    /// The compilation warnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
    /// The factory dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_deps: Option<BTreeMap<String, String>>,
}

impl Contract {
    ///
    /// Returns the signature hash of the specified contract entry.
    ///
    /// # Panics
    /// If the hashes have not been requested in the `vyper` call.
    ///
    pub fn entry(&self, entry: &str) -> u32 {
        self.method_identifiers
            .as_ref()
            .expect("Method identifiers not available")
            .iter()
            .find_map(|(contract_entry, hash)| {
                if contract_entry.starts_with(&(entry.to_owned() + "(")) {
                    Some(
                        u32::from_str_radix(hash.as_str(), era_compiler_common::BASE_HEXADECIMAL)
                            .expect("Test hash is always valid"),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Entry `{entry}` not found"))
    }
}
