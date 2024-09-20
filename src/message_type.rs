//!
//! The compiler message type.
//!

use std::str::FromStr;

///
/// The compiler message type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MessageType {
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
            "txorigin" => Ok(Self::TxOrigin),
            _ => Err(anyhow::anyhow!("Invalid message type: {string}")),
        }
    }
}
