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
use crate::vyper::selector::Selector as VyperSelector;
use crate::warning_type::WarningType;

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
        }
    }

    ///
    /// Parses output lines returned by the Vyper compiler.
    /// The order of `lines` is expected to match that of `selection`.
    ///
    pub fn try_from_lines(
        version: semver::Version,
        source_code: String,
        selection: &[VyperSelector],
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
                VyperSelector::IRJson => {
                    ir = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelector::AST => {
                    ast = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelector::ABI => {
                    abi = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelector::MethodIdentifiers => {
                    method_identifiers = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelector::Layout => {
                    layout = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelector::UserDocumentation => {
                    userdoc = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelector::DeveloperDocumentation => {
                    devdoc = Some(era_compiler_common::deserialize_from_str(line)?);
                }

                VyperSelector::CombinedJson => {
                    panic!("Combined JSON cannot be requested with other types of output");
                }
                VyperSelector::EraVMAssembly => {
                    panic!("EraVM assembly cannot be requested from `vyper` executable");
                }
                VyperSelector::ProjectMetadata => {
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
        no_bytecode_metadata: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_selection: Vec<VyperSelector>,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let version = self.version.clone();

        let warnings = self
            .ast
            .get_warnings(&self.ast.ast, suppressed_warnings.as_slice());

        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings.clone());

        let mut context: era_compiler_llvm_context::EraVMContext =
            era_compiler_llvm_context::EraVMContext::new(
                &llvm,
                llvm.create_module(contract_path),
                llvm_options,
                optimizer,
                debug_config,
            );

        let ir = if output_selection.contains(&VyperSelector::IRJson) {
            Some(self.ir.clone())
        } else {
            None
        };
        let ast = if output_selection.contains(&VyperSelector::AST) {
            Some(self.ast.clone())
        } else {
            None
        };
        let abi = if output_selection.contains(&VyperSelector::ABI) {
            Some(self.abi.clone())
        } else {
            None
        };
        let method_identifiers = if output_selection.contains(&VyperSelector::MethodIdentifiers) {
            Some(self.method_identifiers.clone())
        } else {
            None
        };
        let layout = if output_selection.contains(&VyperSelector::Layout) {
            self.layout.take()
        } else {
            None
        };
        let userdoc = if output_selection.contains(&VyperSelector::UserDocumentation) {
            self.userdoc.take()
        } else {
            None
        };
        let devdoc = if output_selection.contains(&VyperSelector::DeveloperDocumentation) {
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

        let cbor_data = if no_bytecode_metadata {
            None
        } else {
            let cbor_key = crate::r#const::VYPER_PRODUCTION_NAME.to_owned();
            let cbor_data = vec![
                (
                    crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
                    crate::r#const::version().parse().expect("Always valid"),
                ),
                (crate::r#const::VYPER_PRODUCTION_NAME.to_owned(), version),
            ];
            Some((cbor_key, cbor_data))
        };

        let is_minimal_proxy_used = context
            .vyper()
            .expect("Always exists")
            .is_minimal_proxy_used();
        let mut build = context.build(
            contract_path,
            metadata_hash,
            cbor_data,
            output_selection.contains(&VyperSelector::EraVMAssembly)
                || output_selection.contains(&VyperSelector::CombinedJson),
            false,
        )?;

        if is_minimal_proxy_used {
            build.factory_dependencies.insert(
                hex::encode(
                    crate::r#const::MINIMAL_PROXY_BUILD
                        .bytecode_hash
                        .expect("Always exists")
                        .as_slice(),
                ),
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
            );
        }

        Ok(ContractBuild::new(
            build,
            ir,
            ast,
            abi,
            method_identifiers,
            layout,
            userdoc,
            devdoc,
            warnings,
        ))
    }
}

impl EraVMWriteLLVM for Contract {
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext,
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
        context: &mut era_compiler_llvm_context::EraVMContext,
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
