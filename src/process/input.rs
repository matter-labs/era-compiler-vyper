//!
//! Process for compiling a single compilation unit.
//!
//! The input data.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::Contract;
use crate::warning_type::WarningType;

///
/// The input data.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    /// The contract full path.
    pub full_path: String,
    /// The contract representation.
    pub contract: Contract,
    /// The source code hash.
    pub source_code_hash: Option<[u8; compiler_common::BYTE_LENGTH_FIELD]>,
    /// Enables the test bytecode encoding.
    pub enable_test_encoding: bool,
    /// The optimizer settings.
    pub optimizer_settings: compiler_llvm_context::OptimizerSettings,
    /// The suppressed warnings.
    pub suppressed_warnings: Vec<WarningType>,
    /// The debug output config.
    pub debug_config: Option<compiler_llvm_context::DebugConfig>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        full_path: String,
        contract: Contract,
        source_code_hash: Option<[u8; compiler_common::BYTE_LENGTH_FIELD]>,
        enable_test_encoding: bool,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            full_path,
            contract,
            source_code_hash,
            enable_test_encoding,
            optimizer_settings,
            suppressed_warnings,
            debug_config,
        }
    }
}
