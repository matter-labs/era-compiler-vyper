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
    /// The LLVM extra options.
    #[serde(skip_serializing)]
    pub llvm_options: Option<Vec<String>>,
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
        llvm_options: Vec<String>,
    ) -> Self {
        Self {
            evm_version,
            output_selection,
            optimize,
            fallback_to_optimizing_for_size: Some(fallback_to_optimizing_for_size),
            llvm_options: Some(llvm_options),
        }
    }
}
