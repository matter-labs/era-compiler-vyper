//!
//! The `vyper --standard-json` optimizer setting.
//!

///
/// The `vyper --standard-json` optimizer setting.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Optimize {
    /// No optimizations.
    #[serde(rename = "none")]
    None,
    /// Optimizing for gas usage.
    #[serde(rename = "gas")]
    Gas,
    /// Optimizing for bytecode size.
    #[serde(rename = "codesize")]
    Size,

    /// Old boolean option for compatibility.
    #[serde(rename = "false")]
    False,
    /// Old boolean option for compatibility.
    #[serde(rename = "true")]
    True,
}

impl std::fmt::Display for Optimize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Gas => write!(f, "gas"),
            Self::Size => write!(f, "codesize"),

            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
        }
    }
}
