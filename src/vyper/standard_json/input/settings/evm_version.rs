//!
//! The `vyper --standard-json` EVM version.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The `vyper --standard-json` EVM version.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum EVMVersion {
    /// The corresponding EVM version.
    #[serde(rename = "byzantium")]
    Byzantium,
    /// The corresponding EVM version.
    #[serde(rename = "constantinople")]
    Constantinople,
    /// The corresponding EVM version.
    #[serde(rename = "petersburg")]
    Petersburg,
    /// The corresponding EVM version.
    #[serde(rename = "istanbul")]
    Istanbul,
    /// The corresponding EVM version.
    #[serde(rename = "berlin")]
    Berlin,
    /// The corresponding EVM version.
    #[serde(rename = "paris")]
    Paris,
}

impl Default for EVMVersion {
    fn default() -> Self {
        Self::Paris
    }
}

impl std::fmt::Display for EVMVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byzantium => write!(f, "byzantium"),
            Self::Constantinople => write!(f, "constantinople"),
            Self::Petersburg => write!(f, "petersburg"),
            Self::Istanbul => write!(f, "istanbul"),
            Self::Berlin => write!(f, "berlin"),
            Self::Paris => write!(f, "paris"),
        }
    }
}
