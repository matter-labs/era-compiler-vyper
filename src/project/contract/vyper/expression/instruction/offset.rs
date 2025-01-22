//!
//! The offset instructions.
//!

use inkwell::values::BasicValue;

use era_compiler_llvm_context::IContext;

///
/// Translates the Vyper LLL-specific `ceil32` instruction.
///
pub fn ceil_32<'ctx>(
    context: &mut era_compiler_llvm_context::EraVMContext<'ctx>,
    value: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>> {
    let remainder = context.builder().build_int_unsigned_rem(
        value,
        context.field_const(era_compiler_common::BYTE_LENGTH_FIELD as u64),
        "ceil32_remainder",
    )?;
    let shift = context.builder().build_int_sub(
        context.field_const(era_compiler_common::BYTE_LENGTH_FIELD as u64),
        remainder,
        "ceil32_shift",
    )?;
    let shift_remainder = context.builder().build_int_unsigned_rem(
        shift,
        context.field_const(era_compiler_common::BYTE_LENGTH_FIELD as u64),
        "ceil32_shift_remainder",
    )?;
    let result = context
        .builder()
        .build_int_add(value, shift_remainder, "ceil32_ceiled")?;
    Ok(result.as_basic_value_enum())
}
