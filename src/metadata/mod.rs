//!
//! The Vyper metadata.
//!

pub mod function;

use std::collections::BTreeMap;

use self::function::Function;

///
/// The Vyper metadata.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    /// The functions metadata.
    pub function_info: BTreeMap<String, Function>,
}
