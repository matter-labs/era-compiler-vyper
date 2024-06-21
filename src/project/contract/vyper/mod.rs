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
    /// The source metadata.
    pub source_metadata: SourceMetadata,
    /// The inner expression.
    pub expression: Expression,
    /// The contract ABI data.
    pub abi: BTreeMap<String, String>,
    /// The contract AST.
    pub ast: AST,
    /// The dependency data.
    pub dependency_data: DependencyData,
}

impl Contract {
    /// The number of vyper compiler output lines per contract.
    pub const EXPECTED_LINES: usize = 4;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        version: semver::Version,
        source_code: String,
        source_metadata: SourceMetadata,
        expression: Expression,
        abi: BTreeMap<String, String>,
        ast: AST,
    ) -> Self {
        Self {
            version,
            source_code,
            source_metadata,
            expression,
            abi,
            ast,
            dependency_data: DependencyData::default(),
        }
    }

    ///
    /// Parses three lines with JSONs, returned by the Vyper compiler.
    /// The order must be:
    /// 1. The LLL IR JSON
    /// 2. The contract functions metadata
    /// 3. The contract ABI data
    /// 4. The contract AST
    ///
    pub fn try_from_lines(
        version: semver::Version,
        source_code: String,
        mut lines: Vec<&str>,
    ) -> anyhow::Result<Self> {
        if lines.len() != Self::EXPECTED_LINES {
            anyhow::bail!(
                "Expected {} lines with JSONs, found {}",
                Self::EXPECTED_LINES,
                lines.len()
            );
        }

        let expression: Expression = era_compiler_common::deserialize_from_str(lines.remove(0))?;
        let metadata: SourceMetadata = serde_json::from_str(lines.remove(0))?;
        let abi: BTreeMap<String, String> = serde_json::from_str(lines.remove(0))?;
        let ast: AST = serde_json::from_str(lines.remove(0))?;

        Ok(Self::new(
            version,
            source_code,
            metadata,
            expression,
            abi,
            ast,
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
                crate::r#const::FORWARDER_CONTRACT_HASH.clone(),
                crate::r#const::MINIMAL_PROXY_CONTRACT_NAME.to_owned(),
            );
        }

        Ok(ContractBuild::new(build, warnings))
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
                .expect("Always exists")
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
            self.expression.extract_runtime_code()?.unwrap_or_default();
        let mut deploy_code = self.expression.try_into_deploy_code()?;

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
                        if metadata_label == function.ir_identifier.as_str() {
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
            functions.push((Function::new(label, metadata, expression), code_type));
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
        Ok(crate::r#const::FORWARDER_CONTRACT_HASH.clone())
    }

    fn resolve_path(&self, _identifier: &str) -> anyhow::Result<String> {
        anyhow::bail!("Dependency mechanism is not available in Vyper");
    }

    fn resolve_library(&self, _path: &str) -> anyhow::Result<String> {
        anyhow::bail!("Dependency mechanism is not available in Vyper");
    }
}
