//!
//! Process for compiling a single compilation unit.
//!
//! The output data.
//!

use crate::build::contract::Contract as ContractBuild;

///
/// The output data.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// The contract build.
    pub build: ContractBuild,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: ContractBuild) -> Self {
        Self { build }
    }
}
