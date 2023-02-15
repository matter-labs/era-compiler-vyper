//!
//! The `vyper --standard-json` expected output selection.
//!

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

///
/// The `vyper --standard-json` expected output selection.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum Selection {
    /// The function signature hashes JSON representation.
    #[serde(rename = "evm.methodIdentifiers")]
    MethodIdentifiers,
    /// The LLL IR.
    #[serde(rename = "ir")]
    LLL,
}

impl Selection {
    ///
    /// Generates the default output selection pattern.
    ///
    pub fn generate_default() -> BTreeMap<String, Vec<Selection>> {
        let mut map = BTreeMap::new();
        map.insert("*".to_owned(), vec![Self::MethodIdentifiers, Self::LLL]);
        map
    }
}

impl std::fmt::Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MethodIdentifiers => write!(f, "evm.methodIdentifiers"),
            Self::LLL => write!(f, "ir"),
        }
    }
}
