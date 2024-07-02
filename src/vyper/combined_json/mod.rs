//!
//! The `vyper --combined-json` output.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use self::contract::Contract;

///
/// The `vyper --combined-json` output.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CombinedJson {
    /// The contract entries.
    #[serde(flatten)]
    pub contracts: BTreeMap<String, Contract>,
    /// The `vyper` compiler version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The `zkvyper` compiler version.
    pub zk_version: String,
}

impl CombinedJson {
    ///
    /// A shortcut constructor.
    ///
    /// Contracts with ABI and method identifiers must be provided here.
    ///
    pub fn new(
        contracts: BTreeMap<String, Contract>,
        version: Option<&semver::Version>,
        zk_version: &semver::Version,
    ) -> Self {
        Self {
            contracts,
            version: version.map(|version| version.to_string()),
            zk_version: zk_version.to_string(),
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
        file_path.push(format!("combined.{}", era_compiler_common::EXTENSION_JSON));

        if file_path.exists() && !overwrite {
            anyhow::bail!(
                "Refusing to overwrite an existing file {file_path:?} (use --overwrite to force)."
            );
        }

        std::fs::create_dir_all(output_directory)?;
        File::create(&file_path)
            .map_err(|error| anyhow::anyhow!("File {:?} creating error: {}", file_path, error))?
            .write_all(serde_json::to_vec(&self).expect("Always valid").as_slice())
            .map_err(|error| anyhow::anyhow!("File {:?} writing error: {}", file_path, error))?;

        Ok(())
    }
}
