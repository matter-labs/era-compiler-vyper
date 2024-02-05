//!
//! The `assert` instruction.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `assert` instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assert([Box<Expression>; 1]);

impl Assert {
    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<D>(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()>
    where
        D: era_compiler_llvm_context::EraVMDependency + Clone,
    {
        let [condition] = self.0;

        let error_block = context.append_basic_block("if_error");
        let join_block = context.append_basic_block("if_join");

        let condition = condition
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
        context.build_conditional_branch(condition, join_block, error_block);

        context.set_basic_block(error_block);
        context.build_exit(
            context.llvm_runtime().revert,
            context.field_const(0),
            context.field_const(0),
        );

        context.set_basic_block(join_block);

        Ok(())
    }
}
