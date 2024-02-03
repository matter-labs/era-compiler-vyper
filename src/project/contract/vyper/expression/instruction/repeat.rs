//!
//! The `repeat` instruction.
//!

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `repeat` instruction.
///
/// The instruction describes a well-known for-loop.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repeat(Vec<Expression>);

impl Repeat {
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
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<D>(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()>
    where
        D: era_compiler_llvm_context::EraVMDependency + Clone,
    {
        let index_identifier = self.0.remove(0).try_into_identifier()?;
        let start = self.0.remove(0);
        let rounds = self.0.remove(0);
        let _rounds_bound = self.0.remove(0);
        let body = self.0.remove(0);

        let condition_block = context.append_basic_block("repeat_condition");
        let body_block = context.append_basic_block("repeat_body");
        let increment_block = context.append_basic_block("repeat_increment");
        let join_block = context.append_basic_block("repeat_join");

        let start = start.into_llvm_value(context)?.expect("Always exists");
        let rounds = rounds.into_llvm_value(context)?.expect("Always exists");
        let rounds_pointer = context.build_alloca(context.field_type(), "repeat_rounds");
        context.build_store(rounds_pointer, context.field_const(0));

        let index_pointer = context.build_alloca(context.field_type(), "repeat_index_pointer");
        context.build_store(index_pointer, start);
        context
            .current_function()
            .borrow_mut()
            .insert_stack_pointer(index_identifier.clone(), index_pointer);
        context.build_unconditional_branch(condition_block);

        context.set_basic_block(condition_block);
        let rounds_value = context.build_load(rounds_pointer, "repeat_condition_rounds_value");
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::ULT,
            rounds_value.into_int_value(),
            rounds.into_int_value(),
            "repeat_condition_compared",
        );
        context.build_conditional_branch(condition, body_block, join_block);

        context.push_loop(body_block, increment_block, join_block);

        context.set_basic_block(body_block);
        body.into_llvm_value(context)?;
        context.build_unconditional_branch(increment_block);

        context.set_basic_block(increment_block);
        let index_value = context.build_load(index_pointer, "repeat_increment_index_value");
        let index_value_incremented = context.builder().build_int_add(
            index_value.into_int_value(),
            context.field_const(1),
            "repeat_increment_index_value_incremented",
        );
        context.build_store(index_pointer, index_value_incremented);

        let rounds_value = context.build_load(rounds_pointer, "repeat_increment_rounds_value");
        let rounds_value_incremented = context.builder().build_int_add(
            rounds_value.into_int_value(),
            context.field_const(1),
            "repeat_rounds_value_incremented",
        );
        context.build_store(rounds_pointer, rounds_value_incremented);
        context.build_unconditional_branch(condition_block);

        context.pop_loop();
        context
            .current_function()
            .borrow_mut()
            .remove_stack_pointer(index_identifier.as_str());
        context.set_basic_block(join_block);

        Ok(())
    }
}
