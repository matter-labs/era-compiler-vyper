//!
//! The `vyper --standard-json` output error.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The `vyper --standard-json` output error.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// The error message.
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
