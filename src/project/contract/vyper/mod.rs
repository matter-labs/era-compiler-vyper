//!
//! The Vyper contract.
//!

pub mod ast;
pub mod expression;
pub mod function;

use std::collections::BTreeMap;

use era_compiler_llvm_context::EraVMWriteLLVM;
use era_compiler_llvm_context::IContext;

use crate::build::contract::Contract as ContractBuild;
use crate::message_type::MessageType;
use crate::project::dependency_data::DependencyData;
use crate::vyper::selection::Selection as VyperSelection;

use self::ast::AST;
use self::expression::Expression;
use self::function::Function;

///
/// The Vyper contract.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The Vyper compiler version.
    pub version: semver::Version,
    /// The Vyper contract source code.
    pub source_code: String,
    /// The LLL IR parsed from JSON.
    pub ir: Expression,
    /// The contract AST.
    pub ast: AST,
    /// The contract ABI.
    pub abi: serde_json::Value,
    /// The contract method identifiers.
    pub method_identifiers: BTreeMap<String, String>,
    /// The contract storage layout.
    pub layout: Option<serde_json::Value>,
    /// The contract user documentation.
    pub userdoc: Option<serde_json::Value>,
    /// The contract developer documentation.
    pub devdoc: Option<serde_json::Value>,
    /// The dependency data.
    pub dependency_data: DependencyData,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        version: semver::Version,
        source_code: String,
        ir: Expression,
        ast: AST,
        abi: serde_json::Value,
        method_identifiers: BTreeMap<String, String>,
        layout: Option<serde_json::Value>,
        userdoc: Option<serde_json::Value>,
        devdoc: Option<serde_json::Value>,
    ) -> Self {
        Self {
            version,
            source_code,
            ir,
            ast,
            abi,
            method_identifiers,
            layout,
            userdoc,
            devdoc,
            dependency_data: DependencyData::default(),
        }
    }

    ///
    /// Parses output lines returned by the Vyper compiler.
    /// The order of `lines` is expected to match that of `selection`.
    ///
    pub fn try_from_lines(
        version: semver::Version,
        source_code: String,
        selection: &[VyperSelection],
        lines: Vec<&str>,
    ) -> anyhow::Result<Self> {
        let mut ir = None;
        let mut ast = None;
        let mut abi = None;
        let mut method_identifiers = None;
        let mut layout = None;
        let mut userdoc = None;
        let mut devdoc = None;

        for (line, selection) in lines.into_iter().zip(selection) {
            match selection {
                VyperSelection::IRJson => {
                    ir = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::AST => {
                    ast = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::ABI => {
                    abi = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::MethodIdentifiers => {
                    method_identifiers = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::StorageLayout => {
                    layout = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::UserDocumentation => {
                    userdoc = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::DeveloperDocumentation => {
                    devdoc = Some(era_compiler_common::deserialize_from_str(line)?);
                }

                VyperSelection::CombinedJson => {
                    panic!("Combined JSON cannot be requested with other types of output");
                }
                VyperSelection::EraVMAssembly => {
                    panic!("EraVM assembly cannot be requested from `vyper` executable");
                }
                VyperSelection::ProjectMetadata => {
                    panic!("Project metadata cannot be requested from `vyper` executable");
                }
            }
        }

        Ok(Self::new(
            version,
            source_code,
            ir.expect("Always exists"),
            ast.expect("Always exists"),
            abi.expect("Always exists"),
            method_identifiers.expect("Always exists"),
            layout,
            userdoc,
            devdoc,
        ))
    }

    ///
    /// Compiles the contract, returning the build.
    ///
    pub fn compile(
        mut self,
        contract_path: &str,
        metadata_hash: Option<era_compiler_common::Hash>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_selection: Vec<VyperSelection>,
        suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let warnings = self
            .ast
            .get_messages(&self.ast.ast, suppressed_messages.as_slice());

        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings.clone());

        let mut context = era_compiler_llvm_context::EraVMContext::<
            era_compiler_llvm_context::DummyDependency,
        >::new(
            &llvm,
            llvm.create_module(contract_path),
            llvm_options,
            optimizer,
            debug_config,
        );

        let ir = if output_selection.contains(&VyperSelection::IRJson) {
            Some(self.ir.clone())
        } else {
            None
        };
        let ast = if output_selection.contains(&VyperSelection::AST) {
            Some(self.ast.clone())
        } else {
            None
        };
        let abi = if output_selection.contains(&VyperSelection::ABI) {
            Some(self.abi.clone())
        } else {
            None
        };
        let method_identifiers = if output_selection.contains(&VyperSelection::MethodIdentifiers) {
            Some(self.method_identifiers.clone())
        } else {
            None
        };
        let layout = if output_selection.contains(&VyperSelection::StorageLayout) {
            self.layout.take()
        } else {
            None
        };
        let userdoc = if output_selection.contains(&VyperSelection::UserDocumentation) {
            self.userdoc.take()
        } else {
            None
        };
        let devdoc = if output_selection.contains(&VyperSelection::DeveloperDocumentation) {
            self.devdoc.take()
        } else {
            None
        };

        self.declare(&mut context).map_err(|error| {
            anyhow::anyhow!(
                "The contract `{}` LLVM IR generator declaration pass error: {}",
                contract_path,
                error
            )
        })?;
        self.into_llvm(&mut context).map_err(|error| {
            anyhow::anyhow!(
                "The contract `{}` LLVM IR generator definition pass error: {}",
                contract_path,
                error
            )
        })?;

        let is_minimal_proxy_used = context
            .vyper()
            .expect("Always exists")
            .is_minimal_proxy_used();
        let mut build = context.build(
            contract_path,
            metadata_hash,
            output_selection.contains(&VyperSelection::EraVMAssembly),
            false,
        )?;

        if is_minimal_proxy_used {
            build.factory_dependencies.insert(
                hex::encode(crate::r#const::MINIMAL_PROXY_CONTRACT.1.as_slice()),
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
            );
        }

        Ok(ContractBuild::new(
            build,
            ir,
            ast,
            method_identifiers,
            abi,
            layout,
            userdoc,
            devdoc,
            warnings,
        ))
    }
}

impl<D> EraVMWriteLLVM<D> for Contract
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let mut entry = era_compiler_llvm_context::EraVMEntryFunction::default();
        entry.declare(context)?;

        let mut runtime = era_compiler_llvm_context::EraVMRuntime::default();
        runtime.declare(context)?;

        era_compiler_llvm_context::EraVMDeployCodeFunction::new(
            era_compiler_llvm_context::EraVMDummyLLVMWritable::default(),
        )
        .declare(context)?;
        era_compiler_llvm_context::EraVMRuntimeCodeFunction::new(
            era_compiler_llvm_context::EraVMDummyLLVMWritable::default(),
        )
        .declare(context)?;

        for name in [
            era_compiler_llvm_context::EraVMRuntime::FUNCTION_DEPLOY_CODE,
            era_compiler_llvm_context::EraVMRuntime::FUNCTION_RUNTIME_CODE,
            era_compiler_llvm_context::EraVMRuntime::FUNCTION_ENTRY,
        ]
        .into_iter()
        {
            context
                .get_function(name)
                .ok_or_else(|| anyhow::anyhow!("Function `{name}` does not exist"))?
                .borrow_mut()
                .set_vyper_data(era_compiler_llvm_context::EraVMFunctionVyperData::default());
        }

        entry.into_llvm(context)?;

        runtime.into_llvm(context)?;

        Ok(())
    }

    fn into_llvm(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let (mut runtime_code, immutables_size) =
            self.ir.extract_runtime_code()?.unwrap_or_default();
        let mut deploy_code = self.ir.try_into_deploy_code()?;

        match immutables_size {
            Expression::IntegerLiteral(number) => {
                let immutables_size = number
                    .as_u64()
                    .ok_or_else(|| anyhow::anyhow!("Immutable size `{number}` parsing error"))?;
                let vyper_data = era_compiler_llvm_context::EraVMContextVyperData::new(
                    immutables_size as usize,
                    false,
                );
                context.set_vyper_data(vyper_data);
            }
            expression => anyhow::bail!("Invalid immutables size format: {expression:?}"),
        }

        let mut function_expressions = deploy_code
            .extract_functions()?
            .into_iter()
            .map(|(label, expression)| {
                (label, expression, era_compiler_common::CodeSegment::Deploy)
            })
            .collect::<Vec<(String, Expression, era_compiler_common::CodeSegment)>>();
        function_expressions.extend(
            runtime_code
                .extract_functions()?
                .into_iter()
                .map(|(label, expression)| {
                    (label, expression, era_compiler_common::CodeSegment::Runtime)
                })
                .collect::<Vec<(String, Expression, era_compiler_common::CodeSegment)>>(),
        );

        let mut functions = Vec::with_capacity(function_expressions.capacity());
        for (label, expression, code_segment) in function_expressions.into_iter() {
            functions.push((
                Function::new(Expression::safe_label(label.as_str()), expression),
                code_segment,
            ));
        }
        for (function, _code_segment) in functions.iter_mut() {
            function.declare(context)?;
        }
        for (function, code_segment) in functions.into_iter() {
            context.set_code_segment(code_segment);
            function.into_llvm(context)?;
        }

        era_compiler_llvm_context::EraVMDeployCodeFunction::new(deploy_code).into_llvm(context)?;
        era_compiler_llvm_context::EraVMRuntimeCodeFunction::new(runtime_code)
            .into_llvm(context)?;

        Ok(())
    }
}

impl era_compiler_llvm_context::Dependency for DependencyData {
    fn resolve_path(&self, _identifier: &str) -> anyhow::Result<String> {
        anyhow::bail!("Dependency mechanism is not available in Vyper");
    }
}
