//!
//! The `vyper --standard-json` expected output selection.
//!

use std::collections::BTreeMap;

///
/// The `vyper --standard-json` expected output selection.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Selection {
    /// The AST.
    #[serde(rename = "ast")]
    AST,
    /// The function signature hashes JSON.
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
        map.insert(
            "*".to_owned(),
            vec![Self::MethodIdentifiers, Self::LLL, Self::AST],
        );
        map
    }
}

impl std::fmt::Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AST => write!(f, "ast"),
            Self::MethodIdentifiers => write!(f, "evm.methodIdentifiers"),
            Self::LLL => write!(f, "ir"),
        }
    }
}
