//!
//! The Vyper metadata.
//!

pub mod function;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use self::function::Function;

///
/// The Vyper metadata.
///
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Metadata {
    /// The functions metadata.
    pub function_info: BTreeMap<String, Function>,
}
