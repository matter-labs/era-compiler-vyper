//!
//! Process for compiling a single compilation unit.
//!
//! The input data.
//!

use std::borrow::Cow;

use crate::message_type::MessageType;
use crate::project::contract::Contract;
use crate::vyper::selection::Selection as VyperSelection;

///
/// The input data.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Input<'a> {
    /// The contract full path.
    pub full_path: Cow<'a, String>,
    /// The contract representation.
    pub contract: Cow<'a, Contract>,
    /// The metadata hash type.
    pub metadata_hash_type: era_compiler_common::HashType,
    /// The EVM target version.
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// The output selection flags.
    pub output_selection: Vec<VyperSelection>,
    /// The optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The extra LLVM arguments.
    pub llvm_options: Vec<String>,
    /// The suppressed messages.
    pub suppressed_messages: Vec<MessageType>,
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
        metadata_hash_type: era_compiler_common::HashType,
        evm_version: Option<era_compiler_common::EVMVersion>,
        output_selection: Vec<VyperSelection>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            full_path,
            contract,
            metadata_hash_type,
            evm_version,
            output_selection,
            optimizer_settings,
            llvm_options,
            suppressed_messages,
            debug_config,
        }
    }
}
