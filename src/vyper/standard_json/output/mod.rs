//!
//! The `vyper --standard-json` output.
//!

pub mod contract;
pub mod error;

use std::collections::BTreeMap;

use self::contract::Contract;
use self::error::Error;

///
/// The `vyper --standard-json` output.
///
/// Unlike in the Solidity compiler, it is not passed up to the hardhat plugin, but only used here
/// internally to reduce the number of calls to the `vyper` subprocess.
///
#[derive(Debug, serde::Deserialize)]
pub struct Output {
    /// The contracts hashmap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<BTreeMap<String, BTreeMap<String, Contract>>>,
    /// The source code hashmap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<BTreeMap<String, serde_json::Value>>,
    /// The compilation errors and warnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    /// The `vyper` compiler long version.
    #[serde(rename = "compiler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_version: Option<String>,
}
