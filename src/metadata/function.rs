//!
//! The Vyper metadata function.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The Vyper metadata function.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    /// The function name.
    name: String,
    /// The function return type.
    return_type: String,
    /// The identifier in the LLL IR.
    #[serde(rename = "_ir_identifier")]
    ir_identifier: String,
}

impl Function {
    ///
    /// Returns the name.
    ///
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Returns the return type.
    ///
    pub fn return_type(&self) -> &str {
        self.return_type.as_str()
    }

    ///
    /// Returns the normalized IR identifier.
    ///
    pub fn ir_identifier(&self) -> String {
        self.ir_identifier
            .replace(' ', "_")
            .replace(['(', ')', '[', ']', ','], "$")
    }
}
