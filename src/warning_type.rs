//!
//! The compiler warning type.
//!

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

///
/// The compiler warning type.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WarningType {
    /// The warning for eponymous feature.
    EcRecover,
    /// The warning for eponymous feature.
    ExtCodeSize,
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
            "ecrecover" => Ok(Self::EcRecover),
            "extcodesize" => Ok(Self::ExtCodeSize),
            "txorigin" => Ok(Self::TxOrigin),
            _ => Err(anyhow::anyhow!("Invalid warning: {}", string)),
        }
    }
}
