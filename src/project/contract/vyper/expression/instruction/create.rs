//!
//! The `create` instruction adapter.
//!

use era_compiler_llvm_context::IContext;

///
/// Translates various Vyper's `create` built-in instructions.
///
/// If `input_length` is `54`, the built-in is `create_minimal_proxy_to`.
/// If `input_length` is `143`, the built-in is `create_copy_of`.
///
pub fn create<'ctx, D>(
    context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    salt: Option<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: era_compiler_llvm_context::EraVMDependency + Clone,
{
    let create_minimal_proxy_to_block = context.append_basic_block("create_minimal_proxy_to_block");
    let create_from_blueprint_block = context.append_basic_block("create_from_blueprint_block");
    let create_join_block = context.append_basic_block("create_join_block");

    let result_pointer = context.build_alloca(context.field_type(), "create_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    context.builder().build_switch(
        input_length,
        create_from_blueprint_block,
        &[(
            context.field_const(crate::r#const::MINIMAL_PROXY_BUILTIN_INPUT_SIZE as u64),
            create_minimal_proxy_to_block,
        )],
    );

    context.set_basic_block(create_minimal_proxy_to_block);
    let result = create_minimal_proxy_to(context, value, input_offset, salt)?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(create_join_block);

    context.set_basic_block(create_from_blueprint_block);
    let result = create_from_blueprint(context, value, input_offset, input_length, salt)?;
    context.build_store(result_pointer, result);
    context.build_unconditional_branch(create_join_block);

    context.set_basic_block(create_join_block);
    let result = context.build_load(result_pointer, "create_result");
    Ok(result)
}

///
/// Translates the Vyper `create_minimal_proxy_to` built-in input layout into the one expected by EraVM.
///
/// This function extracts the address from the calldata previously assembled in the LLL by the
/// Vyper compiler. Then the address is written to the corresponding offset as the first argument
/// of the minimal proxy's constructor.
///
/// Before Vyper v0.3.4, the built-in was called `create_forwarder_to`.
///
fn create_minimal_proxy_to<'ctx, D>(
    context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    salt: Option<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: era_compiler_llvm_context::EraVMDependency + Clone,
{
    let success_block = context.append_basic_block("create_success_block");
    let failure_block = context.append_basic_block("create_failure_block");
    let join_block = context.append_basic_block("create_join_block");

    let address_offset = context.builder().build_int_add(
        input_offset,
        context.field_const(19),
        "create_address_offset",
    );
    let address_dirty_pointer = era_compiler_llvm_context::Pointer::new_with_offset(
        context,
        era_compiler_llvm_context::EraVMAddressSpace::Heap,
        context.field_type(),
        address_offset,
        "create_address_dirty_pointer",
    );
    let address_dirty = context.build_load(address_dirty_pointer, "create_address_dirty");
    let address = context.builder().build_right_shift(
        address_dirty.into_int_value(),
        context.field_const(
            ((era_compiler_common::BYTE_LENGTH_FIELD
                - era_compiler_common::BYTE_LENGTH_ETH_ADDRESS)
                * era_compiler_common::BIT_LENGTH_BYTE) as u64,
        ),
        false,
        "create_address",
    );

    let calldata_offset =
        context.field_const(era_compiler_llvm_context::eravm_const::HEAP_AUX_OFFSET_EXTERNAL_CALL);
    let calldata_length = context.field_const(
        (era_compiler_llvm_context::eravm_const::DEPLOYER_CALL_HEADER_SIZE
            + era_compiler_common::BYTE_LENGTH_FIELD) as u64,
    );

    let hash_input_offset = context.builder().build_int_add(
        calldata_offset,
        context.field_const(
            (era_compiler_common::BYTE_LENGTH_X32 + era_compiler_common::BYTE_LENGTH_FIELD) as u64,
        ),
        "create_hash_input_offset",
    );
    let hash_input_offset_pointer = era_compiler_llvm_context::Pointer::new_with_offset(
        context,
        era_compiler_llvm_context::EraVMAddressSpace::HeapAuxiliary,
        context.field_type(),
        hash_input_offset,
        "create_hash_input_offset_pointer",
    );
    let hash = context.compile_dependency(crate::r#const::MINIMAL_PROXY_CONTRACT_NAME)?;
    context.build_store(
        hash_input_offset_pointer,
        context.field_const_str_hex(hash.as_str()),
    );

    let address_input_offset = context.builder().build_int_add(
        calldata_offset,
        context
            .field_const(era_compiler_llvm_context::eravm_const::DEPLOYER_CALL_HEADER_SIZE as u64),
        "create_address_input_offset",
    );
    let address_input_offset_pointer = era_compiler_llvm_context::Pointer::new_with_offset(
        context,
        era_compiler_llvm_context::EraVMAddressSpace::HeapAuxiliary,
        context.field_type(),
        address_input_offset,
        "create_address_input_offset_pointer",
    );
    context.build_store(address_input_offset_pointer, address);

    let result_pointer = context.build_alloca(context.field_type(), "create_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    let address_or_status_code = match salt {
        Some(salt) => era_compiler_llvm_context::eravm_evm_create::create2(
            context,
            era_compiler_llvm_context::EraVMAddressSpace::HeapAuxiliary,
            value,
            calldata_offset,
            calldata_length,
            Some(salt),
        ),
        None => era_compiler_llvm_context::eravm_evm_create::create(
            context,
            era_compiler_llvm_context::EraVMAddressSpace::HeapAuxiliary,
            value,
            calldata_offset,
            calldata_length,
        ),
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
    let return_data_size = context
        .get_global_value(era_compiler_llvm_context::eravm_const::GLOBAL_RETURN_DATA_SIZE)?;
    era_compiler_llvm_context::eravm_evm_return_data::copy(
        context,
        context.field_const(0),
        context.field_const(0),
        return_data_size.into_int_value(),
    )?;
    context.build_exit(
        context.llvm_runtime().revert,
        context.field_const(0),
        return_data_size.into_int_value(),
    );
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "create_result");
    Ok(result)
}

///
/// Translates the Vyper's `create_from_blueprint` built-in.
///
/// Makes use of the `EXTCODECOPY` substituted with `EXTCODEHASH` for EraVM.
///
fn create_from_blueprint<'ctx, D>(
    context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    value: inkwell::values::IntValue<'ctx>,
    input_offset: inkwell::values::IntValue<'ctx>,
    input_length: inkwell::values::IntValue<'ctx>,
    salt: Option<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: era_compiler_llvm_context::EraVMDependency + Clone,
{
    let success_block = context.append_basic_block("create_from_blueprint_success_block");
    let failure_block = context.append_basic_block("create_from_blueprint_failure_block");
    let join_block = context.append_basic_block("create_from_blueprint_join_block");

    let result_pointer =
        context.build_alloca(context.field_type(), "create_from_blueprint_result_pointer");
    context.build_store(result_pointer, context.field_const(0));
    let address_or_status_code = match salt {
        Some(salt) => era_compiler_llvm_context::eravm_evm_create::create2(
            context,
            era_compiler_llvm_context::EraVMAddressSpace::Heap,
            value,
            input_offset,
            input_length,
            Some(salt),
        ),
        None => era_compiler_llvm_context::eravm_evm_create::create(
            context,
            era_compiler_llvm_context::EraVMAddressSpace::Heap,
            value,
            input_offset,
            input_length,
        ),
    }?;
    let address_or_status_code_is_zero = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        address_or_status_code.into_int_value(),
        context.field_const(0),
        "create_from_blueprint_address_or_status_code_is_zero",
    );
    context.build_conditional_branch(address_or_status_code_is_zero, failure_block, success_block);

    context.set_basic_block(success_block);
    context.build_store(result_pointer, address_or_status_code);
    context.build_unconditional_branch(join_block);

    context.set_basic_block(failure_block);
    let return_data_size = context
        .get_global_value(era_compiler_llvm_context::eravm_const::GLOBAL_RETURN_DATA_SIZE)?;
    era_compiler_llvm_context::eravm_evm_return_data::copy(
        context,
        context.field_const(0),
        context.field_const(0),
        return_data_size.into_int_value(),
    )?;
    context.build_exit(
        context.llvm_runtime().revert,
        context.field_const(0),
        return_data_size.into_int_value(),
    );
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    let result = context.build_load(result_pointer, "create_from_blueprint_result");
    Ok(result)
}
