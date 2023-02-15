//!
//! The Vyper LLL-specific `clamp` instructions.
//!

use inkwell::values::BasicValue;

///
/// Translates the two-sides bounded clamp.
///
pub fn ordinary<'ctx, D>(
    context: &mut compiler_llvm_context::Context<'ctx, D>,
    operand_1: inkwell::values::IntValue<'ctx>,
    operand_2: inkwell::values::IntValue<'ctx>,
    operand_3: inkwell::values::IntValue<'ctx>,
    is_signed: bool,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: compiler_llvm_context::Dependency,
{
    let error_block = context.append_basic_block("if_error");
    let join_block = context.append_basic_block("if_join");

    let (predicate_one, predicate_two) = if is_signed {
        (inkwell::IntPredicate::SLE, inkwell::IntPredicate::SGE)
    } else {
        (inkwell::IntPredicate::ULE, inkwell::IntPredicate::UGE)
    };

    let condition_one = context.builder().build_int_compare(
        predicate_one,
        operand_2,
        operand_3,
        "clamp_condition_one",
    );
    let condition_two = context.builder().build_int_compare(
        predicate_two,
        operand_3,
        operand_1,
        "clamp_condition_two",
    );
    let condition = context
        .builder()
        .build_and(condition_one, condition_two, "clamp_condition");
    context.build_conditional_branch(condition, join_block, error_block);

    context.set_basic_block(error_block);
    context.build_exit(
        context.intrinsics().revert,
        context.field_const(0),
        context.field_const(0),
    );

    context.set_basic_block(join_block);

    Ok(operand_2.as_basic_value_enum())
}

///
/// Translates the one-side bounded clamp with predicate.
///
pub fn with_predicate<'ctx, D>(
    context: &mut compiler_llvm_context::Context<'ctx, D>,
    operand_1: inkwell::values::IntValue<'ctx>,
    operand_2: inkwell::values::IntValue<'ctx>,
    predicate: inkwell::IntPredicate,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: compiler_llvm_context::Dependency,
{
    let error_block = context.append_basic_block("clamp_single_error");
    let join_block = context.append_basic_block("clamp_single_join");

    let condition = context.builder().build_int_compare(
        predicate,
        operand_1,
        operand_2,
        "clamp_single_condition",
    );
    context.build_conditional_branch(condition, join_block, error_block);

    context.set_basic_block(error_block);
    context.build_exit(
        context.intrinsics().revert,
        context.field_const(0),
        context.field_const(0),
    );

    context.set_basic_block(join_block);

    Ok(operand_1.as_basic_value_enum())
}
