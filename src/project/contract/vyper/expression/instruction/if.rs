//!
//! The `if` instruction.
//!

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use era_compiler_llvm_context::IContext;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `if` instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct If(Vec<Expression>);

impl If {
    ///
    /// Extracts the functions from the deploy or runtime code.
    ///
    pub fn extract_functions(&mut self) -> anyhow::Result<BTreeMap<String, Expression>> {
        let mut result = self
            .0
            .get_mut(1)
            .expect("Always exists")
            .extract_functions()?;
        if let Some(else_expression) = self.0.get_mut(2) {
            result.extend(else_expression.extract_functions()?);
        }
        Ok(result)
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<'ctx, D>(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
    where
        D: era_compiler_llvm_context::EraVMDependency + Clone,
    {
        let main_block = context.append_basic_block("if_main");
        let join_block = context.append_basic_block("if_join");

        let condition_expression = self.0.remove(0);
        let main_expression = self.0.remove(0);
        let else_expression = self.0.pop();

        let result_pointer = context.build_alloca(context.field_type(), "if_result_pointer");
        context.build_store(result_pointer, context.field_const(0));
        let mut returns_value = false;

        let condition = condition_expression
            .into_llvm_value(context)?
            .expect("Always exists")
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "if_condition_extended",
        );
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "if_condition_compared",
        );

        if let Some(else_expression) = else_expression {
            let else_block = context.append_basic_block("if_else");
            context.build_conditional_branch(condition, main_block, else_block);

            context.set_basic_block(else_block);
            if let Some(argument) = else_expression.into_llvm_value(context)? {
                returns_value = true;
                context.build_store(result_pointer, argument);
            }
            context.build_unconditional_branch(join_block);
        } else {
            context.build_conditional_branch(condition, main_block, join_block);
        }

        context.set_basic_block(main_block);
        if let Some(argument) = main_expression.into_llvm_value(context)? {
            returns_value = true;
            context.build_store(result_pointer, argument);
        }
        context.build_unconditional_branch(join_block);

        context.set_basic_block(join_block);
        if !returns_value {
            return Ok(None);
        }

        let result = context.build_load(result_pointer, "if_result");
        Ok(Some(result))
    }
}
