//!
//! The contract.
//!

pub mod llvm_ir;
pub mod metadata;
pub mod vyper;
pub mod zkasm;

use crate::build::contract::Contract as ContractBuild;

use self::llvm_ir::Contract as LLVMIRContract;
use self::vyper::Contract as VyperContract;
use self::zkasm::Contract as ZKASMContract;

///
/// The contract.
///
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum Contract {
    /// The Vyper contract.
    Vyper(VyperContract),
    /// The LLVM IR contract.
    LLVMIR(LLVMIRContract),
    /// The LLVM IR contract.
    ZKASM(ZKASMContract),
}

impl From<VyperContract> for Contract {
    fn from(inner: VyperContract) -> Self {
        Self::Vyper(inner)
    }
}

impl From<LLVMIRContract> for Contract {
    fn from(inner: LLVMIRContract) -> Self {
        Self::LLVMIR(inner)
    }
}

impl From<ZKASMContract> for Contract {
    fn from(inner: ZKASMContract) -> Self {
        Self::ZKASM(inner)
    }
}

impl Contract {
    ///
    /// Compiles the contract, returning the build.
    ///
    pub fn compile(
        self,
        contract_path: &str,
        source_code_hash: [u8; compiler_common::BYTE_LENGTH_FIELD],
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        include_metadata_hash: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        match self {
            Self::Vyper(inner) => inner.compile(
                contract_path,
                source_code_hash,
                target_machine,
                optimizer_settings,
                include_metadata_hash,
                debug_config,
            ),
            Self::LLVMIR(inner) => inner.compile(
                contract_path,
                source_code_hash,
                target_machine,
                optimizer_settings,
                include_metadata_hash,
                debug_config,
            ),
            Self::ZKASM(inner) => inner.compile(
                contract_path,
                source_code_hash,
                optimizer_settings,
                include_metadata_hash,
                debug_config,
            ),
        }
    }
}
