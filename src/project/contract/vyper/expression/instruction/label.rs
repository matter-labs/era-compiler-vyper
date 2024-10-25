//!
//! The `label` instruction.
//!

use std::collections::BTreeMap;

use era_compiler_llvm_context::IContext;
use inkwell::values::BasicValue;

use crate::project::contract::vyper::expression::instruction::Instruction;
use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `label` instruction.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Label(pub Vec<Expression>);

impl Label {
    ///
    /// Extracts the functions from the deploy or runtime code.
    ///
    pub fn extract_functions(&mut self) -> anyhow::Result<BTreeMap<String, Expression>> {
        self.0
            .last_mut()
            .expect("Always exists")
            .extract_functions()
    }

    ///
    /// Whether the label is a function entry block.
    ///
    pub fn is_function_entry(&self) -> anyhow::Result<bool> {
        let label_name = self.name()?;
        Ok(
            label_name.starts_with(crate::r#const::FUNCTION_PREFIX_EXTERNAL)
                || label_name.starts_with(crate::r#const::FUNCTION_PREFIX_INTERNAL),
        )
    }

    ///
    /// Whether the label is a constructor block.
    ///
    pub fn is_constructor_block(label: &str) -> bool {
        label.starts_with(crate::r#const::FUNCTION_PREFIX_EXTERNAL)
            && label.contains(crate::r#const::FUNCTION_NAME_CONSTRUCTOR)
    }

    ///
    /// Checks whether the label is empty. If it is, nothing is generated.
    ///
    pub fn is_empty(&self) -> bool {
        self.0.len() <= 1
    }

    ///
    /// Checks whether the label body is empty. If it is, nothing is generated.
    ///
    /// The cleanup block cannot be ignored in deploy code, because `vyper` generates jumps to them.
    ///
    pub fn can_block_be_ignored(&self) -> bool {
        let label_name = match self.name() {
            Ok(identifier) => identifier,
            Err(_) => return true,
        };

        if Self::is_constructor_block(label_name.as_str())
            && label_name.ends_with(crate::r#const::LABEL_SUFFIX_CLEANUP)
        {
            return false;
        }

        matches!(
            self.0.get(2),
            Some(Expression::Instruction(Instruction::Pass))
        )
    }

    ///
    /// Checks whether the label body is an empty sequence. If it is, a return is appended.
    ///
    /// Only used by the cleanup block in deploy code.
    ///
    pub fn is_block_empty_sequence(&self) -> bool {
        match self.0.get(2) {
            Some(Expression::Instruction(Instruction::Pass)) => true,
            Some(Expression::Instruction(Instruction::Seq(sequence))) => {
                sequence.is_pass_or_empty()
            }
            Some(Expression::Identifier(identifier)) => {
                identifier.as_str() == crate::r#const::DEFAULT_PASS_IDENTIFIER
            }
            Some(_) => false,
            None => true,
        }
    }

    ///
    /// Checks whether the label represents a function with a return value.
    ///
    pub fn has_return_value(&self) -> bool {
        let arguments = match self.0.get(1) {
            Some(Expression::Instruction(Instruction::Var_List(ref arguments))) => arguments,
            Some(_) | None => return false,
        };
        for variable in [
            crate::r#const::VARIABLE_IDENTIFIER_RETURN_PC,
            crate::r#const::VARIABLE_IDENTIFIER_RETURN_BUFFER,
        ]
        .into_iter()
        {
            if !arguments.iter().any(|argument| {
                argument
                    .try_into_identifier()
                    .map(|identifier| identifier.as_str() == variable)
                    .unwrap_or_default()
            }) {
                return false;
            }
        }
        true
    }

    ///
    /// Returns the label name.
    ///
    pub fn name(&self) -> anyhow::Result<String> {
        self.0.first().expect("Always exists").try_into_identifier()
    }

    ///
    /// Declares the label block, so all the blocks are predeclared before translating the bodies.
    ///
    pub fn declare<D>(
        &self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()>
    where
        D: era_compiler_llvm_context::Dependency,
    {
        if self.is_empty() || self.can_block_be_ignored() {
            return Ok(());
        }

        let label_name = self.name()?;
        context.append_basic_block(Expression::safe_label(label_name.as_str()).as_str());

        context.set_basic_block(context.current_function().borrow().entry_block());
        let mut label_arguments = Vec::new();
        match self.0.get(1) {
            Some(Expression::Instruction(Instruction::Var_List(ref arguments))) => {
                for argument in arguments.iter() {
                    let name = argument.try_into_identifier()?;
                    if name.as_str() == crate::r#const::VARIABLE_IDENTIFIER_RETURN_PC {
                        continue;
                    }
                    label_arguments.push(name.clone());

                    let pointer = context.build_alloca(context.field_type(), name.as_str())?;
                    let value =
                        if name.as_str() == crate::r#const::VARIABLE_IDENTIFIER_RETURN_BUFFER {
                            context.current_function().borrow().get_nth_param(0)
                        } else {
                            context.field_const(0).as_basic_value_enum()
                        };
                    context.build_store(pointer, value)?;
                    context
                        .current_function()
                        .borrow_mut()
                        .insert_stack_pointer(name, pointer);
                }
            }
            Some(Expression::Identifier(identifier)) if identifier.as_str() == "var_list" => {}
            expression => anyhow::bail!("Expected a variable list, found `{expression:?}`"),
        };

        context
            .current_function()
            .borrow_mut()
            .vyper_mut()
            .insert_label_arguments(label_name, label_arguments);

        Ok(())
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<D>(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()>
    where
        D: era_compiler_llvm_context::Dependency,
    {
        if self.is_empty() || self.can_block_be_ignored() {
            return Ok(());
        }
        let is_block_empty_sequence = self.is_block_empty_sequence();

        let label_name = self.0.remove(0);
        let block = self.0.remove(1);

        let current_block = context.basic_block();

        let label_name = label_name.try_into_identifier()?;
        let label_block = context
            .current_function()
            .borrow()
            .declaration()
            .value
            .get_basic_blocks()
            .iter()
            .find(|block| {
                block.get_name().to_string_lossy()
                    == Expression::safe_label(label_name.as_str()).as_str()
            })
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Block `{}` does not exist", label_name))?;

        context.set_basic_block(label_block);
        block.into_llvm_value(context)?;

        if label_name == crate::r#const::FUNCTION_IDENTIFIER_FALLBACK {
            context
                .build_unconditional_branch(context.current_function().borrow().return_block())?;
        }

        if Self::is_constructor_block(label_name.as_str())
            && label_name.ends_with(crate::r#const::LABEL_SUFFIX_CLEANUP)
            && is_block_empty_sequence
        {
            era_compiler_llvm_context::eravm_evm_return::stop(context)?;
        }

        context.set_basic_block(current_block);

        Ok(())
    }
}
