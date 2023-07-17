//!
//! The `create` instruction adapter.
//!

///
/// Translates the Vyper LLL `create` input layout into the one expected by zkEVM.
///
/// This function extracts the address from the calldata previously assembled in the LLL by the
/// Vyper compiler. Then the address is written to the corresponding offset as the first argument
/// of the forwarder's constructor.
///
pub fn create<'ctx, D>(
    context: &mut compiler_llvm_context::Context<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    salt: Option<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: compiler_llvm_context::Dependency + Clone,
{
    let success_block = context.append_basic_block("create_success_block");
    let failure_block = context.append_basic_block("create_failure_block");
    let join_block = context.append_basic_block("create_join_block");

    let address_offset = context.builder().build_int_add(
        input_offset,
        context.field_const(19),
        "create_address_offset",
    );
    let address_dirty_pointer = compiler_llvm_context::Pointer::new_with_offset(
        context,
        compiler_llvm_context::AddressSpace::Heap,
        context.field_type(),
        address_offset,
        "create_address_dirty_pointer",
    );
    let address_dirty = context.build_load(address_dirty_pointer, "create_address_dirty");
    let address = context.builder().build_right_shift(
        address_dirty.into_int_value(),
        context.field_const(
            ((compiler_common::BYTE_LENGTH_FIELD - compiler_common::BYTE_LENGTH_ETH_ADDRESS)
                * compiler_common::BIT_LENGTH_BYTE) as u64,
        ),
        false,
        "create_address",
    );

    let calldata_offset = context.field_const(compiler_llvm_context::HEAP_AUX_OFFSET_EXTERNAL_CALL);
    let calldata_length = context.field_const(
        (compiler_llvm_context::DEPLOYER_CALL_HEADER_SIZE + compiler_common::BYTE_LENGTH_FIELD)
            as u64,
    );

    let hash_input_offset = context.builder().build_int_add(
        calldata_offset,
        context.field_const(
            (compiler_common::BYTE_LENGTH_X32 + compiler_common::BYTE_LENGTH_FIELD) as u64,
        ),
        "create_hash_input_offset",
    );
    let hash_input_offset_pointer = compiler_llvm_context::Pointer::new_with_offset(
        context,
        compiler_llvm_context::AddressSpace::HeapAuxiliary,
        context.field_type(),
        hash_input_offset,
        "create_hash_input_offset_pointer",
    );
    let hash = context.compile_dependency(crate::r#const::FORWARDER_CONTRACT_NAME)?;
    context.build_store(
        hash_input_offset_pointer,
        context.field_const_str_hex(hash.as_str()),
    );

    let address_input_offset = context.builder().build_int_add(
        calldata_offset,
        context.field_const(compiler_llvm_context::DEPLOYER_CALL_HEADER_SIZE as u64),
        "create_address_input_offset",
    );
    let address_input_offset_pointer = compiler_llvm_context::Pointer::new_with_offset(
        context,
        compiler_llvm_context::AddressSpace::HeapAuxiliary,
        context.field_type(),
        address_input_offset,
        "create_address_input_offset_pointer",
    );
    context.build_store(address_input_offset_pointer, address);

    let result_pointer = context.build_alloca(context.field_type(), "create_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    let address_or_status_code = match salt {
        Some(salt) => compiler_llvm_context::create::create2(
            context,
            value,
            calldata_offset,
            calldata_length,
            Some(salt),
        ),
        None => {
            compiler_llvm_context::create::create(context, value, calldata_offset, calldata_length)
        }
    }?;
    let address_or_status_code_is_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        address_or_status_code.into_int_value(),
        context.field_const(0),
        "create_address_or_status_code_is_zero",
    );
    context.build_conditional_branch(address_or_status_code_is_zero, failure_block, success_block);

    context.set_basic_block(success_block);
    context.build_store(result_pointer, address_or_status_code);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(failure_block);
    let return_data_size = context.get_global(compiler_llvm_context::GLOBAL_RETURN_DATA_SIZE)?;
    compiler_llvm_context::return_data::copy(
        context,
        context.field_const(0),
        context.field_const(0),
        return_data_size.into_int_value(),
    )?;
    context.build_exit(
        context.intrinsics().revert,
        context.field_const(0),
        return_data_size.into_int_value(),
    );
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "create_result");
    Ok(result)
}
