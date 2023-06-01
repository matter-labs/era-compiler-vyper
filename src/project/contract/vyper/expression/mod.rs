//!
//! The `vyper -f ir_json` output.
//!

pub mod instruction;

use std::collections::BTreeMap;

use inkwell::values::BasicValue;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Number;

use self::instruction::seq::Seq as SeqInstruction;
use self::instruction::Instruction;

///
/// The LLL IR JSON expression.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Expression {
    /// The LLL IR instruction.
    Instruction(Instruction),
    /// The LLL IR integer literal.
    IntegerLiteral(Number),
    /// The LLL IR identifier.
    Identifier(String),

    /// The LLL unknown variant trap.
    Unknown(serde_json::Value),
}

impl Default for Expression {
    fn default() -> Self {
        Self::Instruction(Instruction::Seq(SeqInstruction::default()))
    }
}

impl Expression {
    ///
    /// Extracts the deploy code expression.
    ///
    pub fn try_into_deploy_code(self) -> anyhow::Result<SeqInstruction> {
        match self {
            Self::Instruction(Instruction::Seq(mut sequence)) => {
                sequence.normalize_deploy_code();
                Ok(sequence)
            }
            Self::Instruction(Instruction::Deploy(_deploy)) => {
                let mut sequence = SeqInstruction::default();
                sequence.normalize_deploy_code();
                Ok(sequence)
            }
            instruction => anyhow::bail!("Expected [`seq`, `deploy`], found `{:?}`", instruction),
        }
    }

    ///
    /// Extracts the runtime code expression from the deploy code.
    ///
    pub fn extract_runtime_code(&mut self) -> anyhow::Result<Option<(SeqInstruction, Self)>> {
        match self {
            Self::Instruction(Instruction::Seq(ref mut sequence)) => {
                match sequence.extract_runtime_code()? {
                    Some((mut runtime_code, immutables_size)) => {
                        runtime_code.normalize_runtime_code();
                        Ok(Some((runtime_code, immutables_size)))
                    }
                    None => Ok(None),
                }
            }
            Self::Instruction(Instruction::Deploy(ref mut deploy)) => {
                let (mut runtime_code, immutables_size) = deploy.extract_runtime_code()?;
                runtime_code.normalize_runtime_code();
                Ok(Some((runtime_code, immutables_size)))
            }
            instruction => anyhow::bail!("Expected [`seq`, `deploy`], found `{:?}`", instruction),
        }
    }

    ///
    /// Converts the entity to an identifier.
    ///
    pub fn try_into_identifier(&self) -> anyhow::Result<String> {
        match self {
            Self::Identifier(string) => Ok(string.to_owned()),
            expression => anyhow::bail!("Expected identifier, found `{:?}`", expression),
        }
    }

    ///
    /// Extracts the functions from the deploy or runtime code.
    ///
    pub fn extract_functions(&mut self) -> anyhow::Result<BTreeMap<String, Expression>> {
        match self {
            Self::Instruction(inner) => inner.extract_functions(),
            _ => Ok(BTreeMap::new()),
        }
    }

    ///
    /// Whether the expression is a function entry block.
    ///
    pub fn is_function(&self) -> anyhow::Result<bool> {
        match self {
            Self::Instruction(instruction) => instruction.is_function(),
            _ => Ok(false),
        }
    }

    ///
    /// Returns the function name.
    ///
    pub fn function_name(&self) -> anyhow::Result<String> {
        match self {
            Expression::Instruction(inner) => inner.function_name(),
            expression => anyhow::bail!("Expected a function sequence, found `{:?}`", expression),
        }
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<'ctx, D>(
        self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
    where
        D: compiler_llvm_context::Dependency,
    {
        match self {
            Self::Instruction(inner) => inner.into_llvm_value(context),
            Self::IntegerLiteral(number) => {
                let string = number.to_string();

                let value = if let Some(string) = string.strip_prefix('-') {
                    let modulo = context.field_const_str_dec(string);
                    let max_value_diff = context.builder().build_int_sub(
                        modulo,
                        context.field_const(1),
                        "max_value_diff",
                    );
                    let max_value = context.field_type().const_all_ones();
                    let value = context.builder().build_int_sub(
                        max_value,
                        max_value_diff,
                        "negative_value",
                    );
                    value.as_basic_value_enum()
                } else {
                    context
                        .field_const_str_dec(string.as_str())
                        .as_basic_value_enum()
                };

                Ok(Some(value))
            }
            Self::Identifier(identifier) => {
                if identifier.as_str() == crate::r#const::DEFAULT_SEQUENCE_IDENTIFIER {
                    context.build_exit(
                        context.intrinsics().revert,
                        context.field_const(0),
                        context.field_const(0),
                    );
                }

                let value = match context
                    .current_function()
                    .borrow()
                    .get_stack_pointer(identifier.as_str())
                {
                    Some(pointer) => context.build_load(pointer, identifier.as_str()),
                    None => context.field_const(0).as_basic_value_enum(),
                };

                Ok(Some(value))
            }

            Self::Unknown(value) => {
                anyhow::bail!("Unknown LLL expression: {}", value);
            }
        }
    }
}
