//!
//! The `vyper --standard-json` input language.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The `vyper --standard-json` input language.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// The Vyper language.
    Vyper,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vyper => write!(f, "Vyper"),
        }
    }
}
