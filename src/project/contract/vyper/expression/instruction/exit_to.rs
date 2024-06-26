//!
//! The `exit_to` instruction.
//!

use serde::Deserialize;
use serde::Serialize;

use era_compiler_llvm_context::IContext;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `exit_to` instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExitTo(Vec<Expression>);

impl ExitTo {
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
        let label_name = self.0.remove(0).try_into_identifier()?;
        if label_name.as_str() == crate::r#const::VARIABLE_IDENTIFIER_RETURN_PC {
            context
                .build_unconditional_branch(context.current_function().borrow().return_block())?;
            return Ok(());
        }
        let label_name = label_name
            .strip_prefix(crate::r#const::LABEL_DESTINATION_PREFIX)
            .unwrap_or(label_name.as_str());

        let block = context
            .current_function()
            .borrow()
            .declaration()
            .value
            .get_basic_blocks()
            .iter()
            .find(|block| {
                block.get_name().to_string_lossy() == Expression::safe_label(label_name).as_str()
            })
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Block `{}` does not exist", label_name))?;

        let argument_names = context
            .current_function()
            .borrow()
            .vyper()
            .label_arguments(label_name);
        if let Some(argument_names) = argument_names {
            for (name, expression) in argument_names.into_iter().zip(self.0) {
                let pointer = context
                    .current_function()
                    .borrow()
                    .get_stack_pointer(name.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Variable `{}` not found", name))?;
                let value = expression.into_llvm_value(context)?.expect("Always exists");
                context.build_store(pointer, value)?;
            }
        }

        context.build_unconditional_branch(block)?;
        Ok(())
    }
}
