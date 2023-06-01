//!
//! The `vyper --standard-json` output contract.
//!

pub mod evm;

use serde::Deserialize;

use crate::project::contract::vyper::expression::Expression;

use self::evm::EVM;

///
/// The `vyper --standard-json` output contract.
///
#[derive(Debug, Deserialize)]
pub struct Contract {
    /// The source code.
    pub source_code: Option<String>,
    /// The contract IR code.
    pub ir: Expression,
    /// The contract EVM inner object.
    pub evm: EVM,
}
