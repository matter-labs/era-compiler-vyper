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
use crate::metadata::Metadata as SourceMetadata;
use crate::project::contract::metadata::Metadata as ContractMetadata;
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
    /// The stringified IR.
    pub ir_string: String,
    /// The LLL IR parsed from JSON.
    pub ir: Expression,
    /// The source metadata.
    pub source_metadata: SourceMetadata,
    /// The contract AST.
    pub ast: AST,
    /// The contract ABI.
    pub abi: serde_json::Value,
    /// The contract method identifiers.
    pub method_identifiers: BTreeMap<String, String>,
    /// The contract storage layout.
    pub layout: Option<serde_json::Value>,
    /// The contract interface.
    pub interface: Option<String>,
    /// The contract external interface.
    pub external_interface: Option<String>,
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
        ir_string: String,
        ir: Expression,
        source_metadata: SourceMetadata,
        ast: AST,
        abi: serde_json::Value,
        method_identifiers: BTreeMap<String, String>,
        layout: Option<serde_json::Value>,
        interface: Option<String>,
        external_interface: Option<String>,
        userdoc: Option<serde_json::Value>,
        devdoc: Option<serde_json::Value>,
    ) -> Self {
        Self {
            version,
            source_code,
            ir_string,
            ir,
            source_metadata,
            ast,
            abi,
            method_identifiers,
            layout,
            interface,
            external_interface,
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
        selection: &Vec<VyperSelection>,
        lines: Vec<&str>,
    ) -> anyhow::Result<Self> {
        let mut ir_string = None;
        let mut ir = None;
        let mut metadata = None;
        let mut ast = None;
        let mut abi = None;
        let mut method_identifiers = None;
        let mut layout = None;
        let mut interface = None;
        let mut external_interface = None;
        let mut userdoc = None;
        let mut devdoc = None;
        for (line, selection) in lines.into_iter().zip(selection) {
            match selection {
                VyperSelection::CombinedJson => {
                    panic!("Combined JSON cannot be requested with other types of output")
                }
                VyperSelection::IR => {
                    ir_string = Some(line.to_owned());
                }
                VyperSelection::IRJson => {
                    ir = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::Metadata => {
                    metadata = Some(era_compiler_common::deserialize_from_str(line)?);
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
                VyperSelection::Layout => {
                    layout = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::Interface => {
                    interface = Some(line.to_owned());
                }
                VyperSelection::ExternalInterface => {
                    external_interface = Some(line.to_owned());
                }
                VyperSelection::UserDocumentation => {
                    userdoc = Some(era_compiler_common::deserialize_from_str(line)?);
                }
                VyperSelection::DeveloperDocumentation => {
                    devdoc = Some(era_compiler_common::deserialize_from_str(line)?);
                }
            }
        }

        Ok(Self::new(
            version,
            source_code,
            ir_string.expect("Always exists"),
            ir.expect("Always exists"),
            metadata.expect("Always exists"),
            ast.expect("Always exists"),
            abi.expect("Always exists"),
            method_identifiers.expect("Always exists"),
            layout,
            interface,
            external_interface,
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
        source_code_hash: Option<[u8; era_compiler_common::BYTE_LENGTH_FIELD]>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        suppressed_messages: Vec<MessageType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let warnings = self
            .ast
            .get_messages(&self.ast.ast, suppressed_messages.as_slice());

        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let metadata_hash = source_code_hash.map(|source_code_hash| {
            ContractMetadata::new(
                &source_code_hash,
                &self.version,
                evm_version,
                semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
                optimizer.settings().to_owned(),
                llvm_options.as_slice(),
            )
            .keccak256()
        });

        let dependency_data = DependencyData::default();
        let mut context = era_compiler_llvm_context::EraVMContext::<DependencyData>::new(
            &llvm,
            llvm.create_module(contract_path),
            llvm_options,
            optimizer,
            Some(dependency_data),
            debug_config,
        );

        let abi = std::mem::take(&mut self.abi);
        let method_identifiers = std::mem::take(&mut self.method_identifiers);
        let layout = self.layout.take();
        let userdoc = self.userdoc.take();
        let devdoc = self.devdoc.take();

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

        let is_minimal_proxy_used = context.vyper().is_minimal_proxy_used();
        let mut build = context.build(contract_path, metadata_hash, output_assembly)?;

        if is_minimal_proxy_used {
            build.factory_dependencies.insert(
                crate::r#const::MINIMAL_PROXY_CONTRACT_HASH.clone(),
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
            );
        }

        Ok(ContractBuild::new(
            build,
            Some(method_identifiers),
            Some(abi),
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
                (
                    label,
                    expression,
                    era_compiler_llvm_context::CodeType::Deploy,
                )
            })
            .collect::<Vec<(String, Expression, era_compiler_llvm_context::CodeType)>>();
        function_expressions.extend(
            runtime_code
                .extract_functions()?
                .into_iter()
                .map(|(label, expression)| {
                    (
                        label,
                        expression,
                        era_compiler_llvm_context::CodeType::Runtime,
                    )
                })
                .collect::<Vec<(String, Expression, era_compiler_llvm_context::CodeType)>>(),
        );

        let mut functions = Vec::with_capacity(function_expressions.capacity());
        for (label, expression, code_type) in function_expressions.into_iter() {
            let mut metadata_label = label
                .strip_suffix(format!("_{}", era_compiler_llvm_context::CodeType::Deploy).as_str())
                .unwrap_or(label.as_str());
            metadata_label = label
                .strip_suffix(format!("_{}", era_compiler_llvm_context::CodeType::Runtime).as_str())
                .unwrap_or(metadata_label);
            metadata_label = label
                .strip_suffix(format!("_{}", crate::r#const::LABEL_SUFFIX_COMMON).as_str())
                .unwrap_or(metadata_label);

            let metadata_name =
                self.source_metadata
                    .function_info
                    .iter()
                    .find_map(|(name, function)| {
                        if Expression::safe_label(metadata_label) == function.ir_identifier() {
                            Some(name.to_owned())
                        } else {
                            None
                        }
                    });
            let metadata = match metadata_name {
                Some(metadata_name) => self
                    .source_metadata
                    .function_info
                    .get(metadata_name.as_str())
                    .cloned(),
                None => None,
            };
            functions.push((
                Function::new(Expression::safe_label(label.as_str()), metadata, expression),
                code_type,
            ));
        }
        for (function, _code_type) in functions.iter_mut() {
            function.declare(context)?;
        }
        for (function, code_type) in functions.into_iter() {
            context.set_code_type(code_type);
            function.into_llvm(context)?;
        }

        era_compiler_llvm_context::EraVMDeployCodeFunction::new(deploy_code).into_llvm(context)?;
        era_compiler_llvm_context::EraVMRuntimeCodeFunction::new(runtime_code)
            .into_llvm(context)?;

        Ok(())
    }
}

impl era_compiler_llvm_context::Dependency for DependencyData {
    fn get(&self, _name: &str) -> anyhow::Result<String> {
        Ok(crate::r#const::MINIMAL_PROXY_CONTRACT_HASH.clone())
    }

    fn resolve_path(&self, _identifier: &str) -> anyhow::Result<String> {
        anyhow::bail!("Dependency mechanism is not available in Vyper");
    }

    fn resolve_library(&self, _path: &str) -> anyhow::Result<String> {
        anyhow::bail!("Dependency mechanism is not available in Vyper");
    }
}
