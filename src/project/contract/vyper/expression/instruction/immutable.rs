//!
//! The immutable instructions.
//!

use era_compiler_llvm_context::IContext;

///
/// Translates the immutable load sequence.
///
/// It is a custom Vyper-specific instruction, which is capable of copying an array of immutables
/// from the immutable storage system contract to the heap.
///
pub fn load_bytes<'ctx, D>(
    context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    heap_offset: inkwell::values::IntValue<'ctx>,
    immutable_offset: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<()>
where
    D: era_compiler_llvm_context::EraVMDependency + Clone,
{
    let condition_block = context.append_basic_block("immutable_load_bytes_repeat_condition");
    let body_block = context.append_basic_block("immutable_load_bytes_repeat_body");
    let increment_block = context.append_basic_block("immutable_load_bytes_repeat_increment");
    let join_block = context.append_basic_block("immutable_load_bytes_repeat_join");

    let heap_offset_pointer = context.build_alloca(
        context.field_type(),
        "immutable_load_bytes_heap_offset_pointer",
    );
    context.build_store(heap_offset_pointer, heap_offset);
    let immutable_offset_pointer = context.build_alloca(
        context.field_type(),
        "immutable_load_bytes_immutable_pointer",
    );
    context.build_store(immutable_offset_pointer, immutable_offset);
    let index_offset_pointer = context.build_alloca(
        context.field_type(),
        "immutable_load_bytes_index_offset_pointer",
    );
    context.build_store(index_offset_pointer, context.field_const(0));
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(condition_block);
    let index_offset_value = context.build_load(
        index_offset_pointer,
        "immutable_load_bytes_condition_index_offset_pointer",
    );
    let condition = context.builder().build_int_compare(
        inkwell::IntPredicate::ULT,
        index_offset_value.into_int_value(),
        length,
        "immutable_load_bytes_condition_compared",
    );
    context.build_conditional_branch(condition, body_block, join_block);

    context.set_basic_block(body_block);
    let immutable_offset_value = context.build_load(
        immutable_offset_pointer,
        "immutable_load_bytes_immutable_offset_value",
    );
    let immutable_value = era_compiler_llvm_context::eravm_evm_immutable::load(
        context,
        immutable_offset_value.into_int_value(),
    )?;

    let heap_offset_value = context.build_load(
        heap_offset_pointer,
        "immutable_load_bytes_heap_offset_value",
    );
    let heap_pointer = era_compiler_llvm_context::EraVMPointer::new_with_offset(
        context,
        era_compiler_llvm_context::EraVMAddressSpace::Heap,
        context.field_type(),
        heap_offset_value.into_int_value(),
        "immutable_load_bytes_heap_pointer",
    );
    context.build_store(heap_pointer, immutable_value);
    context.build_unconditional_branch(increment_block);

    context.set_basic_block(increment_block);
    let heap_offset_value = context.build_load(
        heap_offset_pointer,
        "immutable_load_bytes_increment_heap_offset_value",
    );
    let heap_offset_value_incremented = context.builder().build_int_add(
        heap_offset_value.into_int_value(),
        context.field_const(era_compiler_common::BYTE_LENGTH_FIELD as u64),
        "immutable_load_bytes_heap_offset_value_incremented",
    );
    context.build_store(heap_offset_pointer, heap_offset_value_incremented);

    let immutable_offset_value = context.build_load(
        immutable_offset_pointer,
        "immutable_load_bytes_increment_immutable_offset_value",
    );
    let immutable_offset_value_incremented = context.builder().build_int_add(
        immutable_offset_value.into_int_value(),
        context.field_const(era_compiler_common::BYTE_LENGTH_FIELD as u64),
        "immutable_load_bytes_immutable_offset_value_incremented",
    );
    context.build_store(immutable_offset_pointer, immutable_offset_value_incremented);

    let index_offset_value = context.build_load(
        index_offset_pointer,
        "immutable_load_bytes_increment_index_offset_value",
    );
    let index_offset_value_incremented = context.builder().build_int_add(
        index_offset_value.into_int_value(),
        context.field_const(era_compiler_common::BYTE_LENGTH_FIELD as u64),
        "immutable_load_bytes_increment_index_offset_value_incremented",
    );
    context.build_store(index_offset_pointer, index_offset_value_incremented);
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(join_block);

    Ok(())
}
