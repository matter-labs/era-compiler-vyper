//!
//! The contract.
//!

pub mod eravm_assembly;
pub mod llvm_ir;
pub mod metadata;
pub mod vyper;

use crate::build::contract::Contract as ContractBuild;
use crate::vyper::selection::Selection as VyperSelection;
use crate::warning_type::WarningType;

use self::eravm_assembly::Contract as EraVMAssemblyContract;
use self::llvm_ir::Contract as LLVMIRContract;
use self::vyper::Contract as VyperContract;

///
/// The contract.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Contract {
    /// The Vyper contract.
    Vyper(VyperContract),
    /// The LLVM IR contract.
    LLVMIR(LLVMIRContract),
    /// The LLVM IR contract.
    EraVMAssembly(EraVMAssemblyContract),
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

impl From<EraVMAssemblyContract> for Contract {
    fn from(inner: EraVMAssemblyContract) -> Self {
        Self::EraVMAssembly(inner)
    }
}

impl Contract {
    ///
    /// Compiles the contract, returning the build.
    ///
    pub fn compile(
        self,
        contract_path: &str,
        metadata_hash: Option<era_compiler_common::Hash>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_selection: Vec<VyperSelection>,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        match self {
            Self::Vyper(inner) => inner.compile(
                contract_path,
                metadata_hash,
                optimizer_settings,
                llvm_options,
                output_selection,
                suppressed_warnings,
                debug_config,
            ),
            Self::LLVMIR(inner) => inner.compile(
                contract_path,
                metadata_hash,
                optimizer_settings,
                llvm_options,
                output_selection,
                suppressed_warnings,
                debug_config,
            ),
            Self::EraVMAssembly(inner) => inner.compile(
                contract_path,
                metadata_hash,
                optimizer_settings,
                llvm_options,
                output_selection,
                suppressed_warnings,
                debug_config,
            ),
        }
    }

    ///
    /// Returns the source code reference.
    ///
    pub fn source_code(&self) -> &str {
        match self {
            Self::Vyper(inner) => inner.source_code.as_str(),
            Self::LLVMIR(inner) => inner.source_code.as_str(),
            Self::EraVMAssembly(inner) => inner.source_code.as_str(),
        }
    }

    ///
    /// Returns the stringified IR reference.
    ///
    pub fn ir_string(&self) -> Option<String> {
        match self {
            Self::Vyper(inner) => {
                Some(serde_json::to_string_pretty(&inner.ir).expect("Always valid"))
            }
            _ => None,
        }
    }
}
