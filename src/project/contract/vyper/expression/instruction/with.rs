//!
//! The `with` instruction.
//!

use std::collections::BTreeMap;

use era_compiler_llvm_context::IContext;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `with` instruction.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct With([Box<Expression>; 3]);

impl With {
    ///
    /// Extracts the functions from the deploy or runtime code.
    ///
    pub fn extract_functions(&mut self) -> anyhow::Result<BTreeMap<String, Expression>> {
        self.0
            .get_mut(2)
            .expect("Always exists")
            .extract_functions()
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<'ctx>(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<'ctx>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>> {
        let [identifier, value, block] = self.0;
        let identifier = identifier.try_into_identifier()?;

        let pointer = context.build_alloca(context.field_type(), identifier.as_str())?;
        let value = value
            .into_llvm_value(context)?
            .ok_or_else(|| anyhow::anyhow!("Expected a value"))?;
        context.build_store(pointer, value)?;
        let shadowed_pointer = context
            .current_function()
            .borrow_mut()
            .insert_stack_pointer(identifier.clone(), pointer);

        let result = block.into_llvm_value(context)?;

        match shadowed_pointer {
            Some(old_pointer) => {
                context
                    .current_function()
                    .borrow_mut()
                    .insert_stack_pointer(identifier, old_pointer);
            }
            None => {
                context
                    .current_function()
                    .borrow_mut()
                    .remove_stack_pointer(identifier.as_str());
            }
        }

        Ok(result)
    }
}
