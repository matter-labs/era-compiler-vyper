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
    /// The function arguments.
    #[serde(rename = "args")]
    pub arguments: BTreeMap<String, String>,
    /// The function return type.
    pub return_type: String,
    /// The function mutability modifier.
    pub mutability: String,
    /// Whether the function is internal.
    #[serde(rename = "internal")]
    pub is_internal: bool,
    /// The no-reentrancy key.
    pub nonreentrant_key: Option<String>,
    /// Whether the function is from JSON.
    #[serde(rename = "is_from_json")]
    pub from_json: bool,
    /// The base function arguments.
    #[serde(rename = "base_args")]
    pub base_arguments: BTreeMap<String, String>,
    /// The default function arguments.
    #[serde(rename = "default_args")]
    pub default_arguments: BTreeMap<String, String>,
    /// The default function argument values.
    pub default_values: BTreeMap<String, String>,
    /// The identifier in the LLL IR.
    #[serde(rename = "_ir_identifier")]
    pub ir_identifier: String,
}
