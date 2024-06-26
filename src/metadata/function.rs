//!
//! The Vyper metadata function.
//!

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper metadata function.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        Expression::safe_label(self.ir_identifier.as_str())
    }
}
