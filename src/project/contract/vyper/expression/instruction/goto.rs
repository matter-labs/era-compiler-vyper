//!
//! The `goto` instruction.
//!

use era_compiler_llvm_context::IContext;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `goto` instruction.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Goto(Vec<Expression>);

impl Goto {
    ///
    /// Return a `goto` to the specified block.
    ///
    pub fn new_to_block(name: &str) -> Self {
        Self(vec![Expression::Identifier(name.to_string())])
    }

    ///
    /// Generates the function call code.
    ///
    pub fn into_function_call<'ctx>(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
        label_name: String,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>> {
        let function = context
            .get_function(label_name.as_str())
            .ok_or_else(|| anyhow::anyhow!("Function `{label_name}` does not exist"))?;

        let mut arguments = Vec::new();
        for expression in self.0.into_iter() {
            if let Ok(identifier) = expression.try_into_identifier() {
                if identifier.starts_with(crate::r#const::LABEL_DESTINATION_PREFIX) {
                    continue;
                }
            }
            if let Some(value) = expression.into_llvm_value(context)? {
                arguments.push(value);
            }
        }

        context.build_call(
            function.borrow().declaration(),
            arguments.as_slice(),
            label_name.as_str(),
        )?;

        Ok(None)
    }

    ///
    /// Generates the block call code.
    ///
    pub fn into_block_call<'ctx>(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
        label_name: String,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>> {
        let block = context
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

        let argument_names = context
            .current_function()
            .borrow()
            .vyper()
            .label_arguments(label_name.as_str());
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

        Ok(None)
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<'ctx>(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>> {
        let label_name = self.0.remove(0).try_into_identifier()?;

        if label_name.ends_with(crate::r#const::LABEL_SUFFIX_CLEANUP)
            || label_name == crate::r#const::FUNCTION_IDENTIFIER_FALLBACK
        {
            return self.into_block_call(context, label_name);
        }

        self.into_function_call(context, Expression::safe_label(label_name.as_str()))
    }
}
