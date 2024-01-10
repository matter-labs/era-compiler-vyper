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
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(skip_serializing)]
    pub fallback_to_optimizing_for_size: Option<bool>,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        evm_version: EVMVersion,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: bool,
        fallback_to_optimizing_for_size: bool,
    ) -> Self {
        Self {
            evm_version,
            output_selection,
            optimize,
            fallback_to_optimizing_for_size: Some(fallback_to_optimizing_for_size),
        }
    }
}
