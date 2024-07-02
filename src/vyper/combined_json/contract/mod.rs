//!
//! The `vyper --combined-json` contract.
//!

pub mod warning;

use std::collections::BTreeMap;

use self::warning::Warning;

///
/// The contract.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    /// A shortcut constructor.
    ///
    pub fn new(
        method_identifiers: Option<BTreeMap<String, String>>,
        abi: Option<serde_json::Value>,
    ) -> Self {
        Self {
            method_identifiers,
            abi,
            bytecode: None,

            assembly: None,
            warnings: None,
            factory_deps: None,
        }
    }

    ///
    /// Creates a minimal proxy.
    ///
    pub fn new_minimal_proxy(output_assembly: bool) -> Self {
        Self {
            method_identifiers: None,
            abi: None,
            bytecode: Some(hex::encode(
                crate::r#const::MINIMAL_PROXY_CONTRACT_BYTECODE.as_slice(),
            )),

            assembly: if output_assembly {
                Some(crate::r#const::MINIMAL_PROXY_CONTRACT_ASSEMBLY.to_owned())
            } else {
                None
            },
            warnings: Some(Vec::new()),
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
