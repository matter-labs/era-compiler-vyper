//!
//! The `vyper --standard-json` input settings.
//!

pub mod selection;

use std::collections::BTreeMap;

use serde::Serialize;

use self::selection::Selection;

///
/// The `vyper --standard-json` input settings.
///
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The EVM version. The latest is the most lightweight, but must be ignored by `vyper`.
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// The output selection filters.
    pub output_selection: BTreeMap<String, Vec<Selection>>,
    /// Whether the optimizer is enabled.
    pub optimize: bool,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(skip_serializing)]
    pub fallback_to_optimizing_for_size: Option<bool>,
    /// Whether to disable the system request memoization.
    #[serde(skip_serializing)]
    pub disable_system_request_memoization: Option<bool>,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        evm_version: Option<era_compiler_common::EVMVersion>,
        output_selection: BTreeMap<String, Vec<Selection>>,
        optimize: bool,
        fallback_to_optimizing_for_size: bool,
        disable_system_request_memoization: bool,
    ) -> Self {
        Self {
            evm_version,
            output_selection,
            optimize,
            fallback_to_optimizing_for_size: Some(fallback_to_optimizing_for_size),
            disable_system_request_memoization: Some(disable_system_request_memoization),
        }
    }
}
