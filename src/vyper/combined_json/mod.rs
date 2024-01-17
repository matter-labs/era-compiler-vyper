//!
//! The `vyper --combined-json` output.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use self::contract::Contract;

///
/// The `vyper --combined-json` output.
///
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CombinedJson {
    /// The contract entries.
    #[serde(flatten)]
    pub contracts: BTreeMap<String, Contract>,
    /// The `vyper` compiler version.
    pub version: String,
    /// The `zkvyper` compiler version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_version: Option<String>,
}

impl CombinedJson {
    ///
    /// Removes EVM artifacts to prevent their accidental usage.
    ///
    pub fn remove_evm(&mut self) {
        for (_, contract) in self.contracts.iter_mut() {
            contract.bytecode = None;
            contract.bytecode_runtime = None;
        }
    }

    ///
    /// Writes the JSON to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let mut file_path = output_directory.to_owned();
        file_path.push(format!("combined.{}", compiler_common::EXTENSION_JSON));

        if file_path.exists() && !overwrite {
            eprintln!(
                "Refusing to overwrite an existing file {file_path:?} (use --overwrite to force)."
            );
            return Ok(());
        }

        File::create(&file_path)
            .map_err(|error| anyhow::anyhow!("File {:?} creating error: {}", file_path, error))?
            .write_all(serde_json::to_vec(&self).expect("Always valid").as_slice())
            .map_err(|error| anyhow::anyhow!("File {:?} writing error: {}", file_path, error))?;

        Ok(())
    }
}
