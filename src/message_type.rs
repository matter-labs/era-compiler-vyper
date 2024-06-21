//!
//! The compiler message type.
//!

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

///
/// The compiler message type.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageType {
    /// The warning for eponymous feature.
    EcRecover,
    /// The warning for eponymous feature.
    ExtCodeSize,
    /// The warning for eponymous feature.
    TxOrigin,
}

impl MessageType {
    ///
    /// Converts string arguments into an array of messages.
    ///
    pub fn try_from_strings(strings: &[String]) -> Result<Vec<Self>, anyhow::Error> {
        strings
            .iter()
            .map(|string| Self::from_str(string))
            .collect()
    }
}

impl FromStr for MessageType {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "ecrecover" => Ok(Self::EcRecover),
            "extcodesize" => Ok(Self::ExtCodeSize),
            "txorigin" => Ok(Self::TxOrigin),
            _ => Err(anyhow::anyhow!("Invalid message type: {string}")),
        }
    }
}
