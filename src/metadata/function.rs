//!
//! The Vyper metadata function.
//!

use std::collections::BTreeMap;

use serde::Deserialize;

///
/// The Vyper metadata function.
///
#[derive(Debug, Deserialize, Clone)]
pub struct Function {
    /// The function name.
    pub name: String,
    /// The function return type.
    pub return_type: String,
    /// The identifier in the LLL IR.
    #[serde(rename = "_ir_identifier")]
    pub ir_identifier: String,
}
