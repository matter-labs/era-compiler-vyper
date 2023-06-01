//!
//! The `vyper --standard-json` input settings.
//!

pub mod evm_version;
pub mod selection;

use std::collections::BTreeMap;

use serde::Serialize;

use self::evm_version::EVMVersion;
use self::selection::Selection;

///
/// The `vyper --standard-json` input settings.
///
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The EVM version. The latest is the most lightweight, but must be ignored by `vyper`.
    pub evm_version: EVMVersion,
    /// The output selection filters.
    pub output_selection: BTreeMap<String, Vec<Selection>>,
    /// Whether the optimizer is enabled.
    pub optimize: bool,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        evm_version: EVMVersion,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: bool,
    ) -> Self {
        Self {
            evm_version,
            output_selection,
            optimize,
        }
    }
}
