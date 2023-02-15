//!
//! The `vyper --standard-json` output contract.
//!

pub mod evm;

use serde::Deserialize;

use crate::metadata::Metadata;
use crate::project::contract::vyper::expression::Expression;

use self::evm::EVM;

///
/// The `vyper --standard-json` output contract.
///
#[derive(Debug, Deserialize)]
pub struct Contract {
    /// The contract IR code.
    pub ir: Expression,
    /// The contract metadata.
    pub metadata: Metadata,
    /// The contract EVM inner object.
    pub evm: EVM,
}
