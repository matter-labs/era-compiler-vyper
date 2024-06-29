//!
//! The `vyper --standard-json` input language.
//!

///
/// The `vyper --standard-json` input language.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
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
