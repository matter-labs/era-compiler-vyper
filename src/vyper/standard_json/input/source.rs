//!
//! The `vyper --standard-json` input source.
//!

use std::io::Read;
use std::path::Path;

///
/// The `vyper --standard-json` input source.
///
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The source code file content.
    pub content: String,
}

impl From<String> for Source {
    fn from(content: String) -> Self {
        Self { content }
    }
}

impl TryFrom<&Path> for Source {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let content = if path.to_string_lossy() == "-" {
            let mut solidity_code = String::with_capacity(16384);
            std::io::stdin()
                .read_to_string(&mut solidity_code)
                .map_err(|error| anyhow::anyhow!("<stdin> reading error: {}", error))?;
            solidity_code
        } else {
            std::fs::read_to_string(path)
                .map_err(|error| anyhow::anyhow!("File {:?} reading error: {}", path, error))?
        };

        Ok(Self { content })
    }
}
