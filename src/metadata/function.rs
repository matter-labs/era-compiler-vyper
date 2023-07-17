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
    pub name: String,
    /// The function return type.
    pub return_type: String,
    /// The identifier in the LLL IR.
    #[serde(rename = "_ir_identifier")]
    pub ir_identifier: String,
}
