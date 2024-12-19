//!
//! Compiler warning type.
//!

use std::str::FromStr;

///
/// Compiler warning type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WarningType {
    /// The warning for eponymous feature.
    TxOrigin,
}

impl WarningType {
    ///
    /// Converts string arguments into an array of warnings.
    ///
    pub fn try_from_strings(strings: &[String]) -> Result<Vec<Self>, anyhow::Error> {
        strings
            .iter()
            .map(|string| Self::from_str(string))
            .collect()
    }
}

impl FromStr for WarningType {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "txorigin" => Ok(Self::TxOrigin),
            _ => Err(anyhow::anyhow!("invalid warning type: {string}")),
        }
    }
}

impl std::fmt::Display for WarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TxOrigin => write!(f, "txorigin"),
        }
    }
}
