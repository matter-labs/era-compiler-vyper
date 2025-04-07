//!
//! Process for compiling a single compilation unit.
//!
//! The input data.
//!

use std::borrow::Cow;

use crate::project::contract::Contract;
use crate::vyper::selector::Selector as VyperSelector;
use crate::warning_type::WarningType;

///
/// The input data.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Input<'a> {
    /// The contract full path.
    pub full_path: Cow<'a, String>,
    /// The contract representation.
    pub contract: Cow<'a, Contract>,
    /// The metadata hash.
    pub metadata_hash: Option<era_compiler_common::Hash>,
    /// Do not include the metadata in the bytecode.
    pub append_bytecode_metadata: bool,
    /// The output selection flags.
    pub output_selection: Vec<VyperSelector>,
    /// The optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The extra LLVM arguments.
    pub llvm_options: Vec<String>,
    /// The suppressed warnings.
    pub suppressed_warnings: Vec<WarningType>,
    /// The debug output config.
    pub debug_config: Option<era_compiler_llvm_context::DebugConfig>,
}

impl<'a> Input<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        full_path: Cow<'a, String>,
        contract: Cow<'a, Contract>,
        metadata_hash: Option<era_compiler_common::Hash>,
        append_bytecode_metadata: bool,
        output_selection: Vec<VyperSelector>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            full_path,
            contract,
            metadata_hash,
            append_bytecode_metadata,
            output_selection,
            optimizer_settings,
            llvm_options,
            suppressed_warnings,
            debug_config,
        }
    }
}
