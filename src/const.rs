//!
//! Vyper to EraVM compiler constants.
//!

#![allow(dead_code)]

use lazy_static::lazy_static;

/// The default executable name.
pub static DEFAULT_EXECUTABLE_NAME: &str = "zkvyper";

/// The `FREE_VAR_SPACE` offset.
pub const OFFSET_FREE_VAR_SPACE: usize = 0;

/// The `FREE_VAR_SPACE2` offset.
pub const OFFSET_FREE_VAR_SPACE2: usize =
    OFFSET_FREE_VAR_SPACE + era_compiler_common::BYTE_LENGTH_FIELD;

/// The non-reserved memory offset.
pub const OFFSET_NON_RESERVED: usize =
    OFFSET_FREE_VAR_SPACE2 + era_compiler_common::BYTE_LENGTH_FIELD;

/// The default label destination prefix.
pub const LABEL_DESTINATION_PREFIX: &str = "_sym_";

/// The default sequence identifier.
pub const DEFAULT_SEQUENCE_IDENTIFIER: &str = "seq";

/// The default pass identifier.
pub const DEFAULT_PASS_IDENTIFIER: &str = "pass";

/// The internal function prefix.
pub const FUNCTION_PREFIX_INTERNAL: &str = "internal";

/// The external function prefix.
pub const FUNCTION_PREFIX_EXTERNAL: &str = "external";

/// The fallback function identifier.
pub const FUNCTION_IDENTIFIER_FALLBACK: &str = "fallback";

/// The return PC variable identifier.
pub const VARIABLE_IDENTIFIER_RETURN_PC: &str = "return_pc";

/// The return buffer variable identifier.
pub const VARIABLE_IDENTIFIER_RETURN_BUFFER: &str = "return_buffer";

/// The common label suffix.
pub const LABEL_SUFFIX_COMMON: &str = "common";

/// The cleanup label suffix.
pub const LABEL_SUFFIX_CLEANUP: &str = "cleanup";

/// The forbidden function `create_copy_of`.
pub const FORBIDDEN_FUNCTION_NAME_CREATE_COPY_OF: &str = "create_copy_of";

/// The `EXTCODESIZE` argument LLL IR name when the blueprint size is requested.
pub const EXTCODESIZE_BLUEPRINT_ARGUMENT_NAME: &str = "create_target";

/// The `create_minimal_proxy_to` contract name.
pub const MINIMAL_PROXY_CONTRACT_NAME: &str = "__VYPER_MINIMAL_PROXY_CONTRACT";

/// The `create_minimal_proxy_to` contract size that is emitted by the upstream Vyper compiler to CREATE's LLL IR.
/// The value is used to route between several built-in codegen when analyzing the CREATE opcode arguments.
pub const MINIMAL_PROXY_BUILTIN_INPUT_SIZE: usize = 54;

lazy_static! {
    ///
    /// The Vyper minimal proxy bytecode in bytes.
    ///
    pub static ref FORWARDER_CONTRACT_BYTECODE_WORDS: Vec<[u8; era_compiler_common::BYTE_LENGTH_FIELD]> = {
        let mut assembly =
            zkevm_assembly::Assembly::from_string(FORWARDER_CONTRACT_ASSEMBLY.to_owned(), None).expect("Always valid");
        assembly
            .compile_to_bytecode().expect("Always valid")
    };

    ///
    /// The Vyper minimal proxy bytecode in words.
    ///
    pub static ref FORWARDER_CONTRACT_BYTECODE: Vec<u8> = {
        FORWARDER_CONTRACT_BYTECODE_WORDS.clone()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>()
    };

    ///
    /// The Vyper minimal proxy bytecode hash.
    ///
    pub static ref FORWARDER_CONTRACT_HASH: String = {
        zkevm_opcode_defs::bytecode_to_code_hash(FORWARDER_CONTRACT_BYTECODE_WORDS.as_slice()).map(hex::encode)
            .expect("Always valid")
    };
}

/// The minimal proxy contract assembly.
pub const FORWARDER_CONTRACT_ASSEMBLY: &str = r#"
	.text
	.file	"minimal_proxy.sol:Test"
	.globl	__entry
__entry:
.func_begin0:
	nop	stack+=[1]
	add	r1, r0, r3
	shr.s	96, r3, r4
	and	@CPI0_0[0], r4, r3
	ptr.add	r1, r3, stack[@ptr_return_data]
	ptr.add	r1, r0, stack[@ptr_calldata]
	and	@CPI0_0[0], r4, stack[@calldatasize]
	add	0, r0, stack[@returndatasize]
	and!	1, r2, r1
	jump.ne	@.BB0_1
	add	128, r0, r1
	st.1	64, r1
	ptr.add	stack[@ptr_calldata], r0, r1
	add	stack[@calldatasize], r0, r2
	and	31, r2, r3
	shr.s!	5, r2, r2
	jump.eq	@.BB0_28
	add	r0, r0, r4
.BB0_11:
	shl.s	5, r4, r5
	ptr.add	r1, r5, r6
	ld	r6, r6
	st.1	r5, r6
	add	1, r4, r4
	sub!	r4, r2, r5
	jump.lt	@.BB0_11
.BB0_28:
	sub!	r3, r0, r4
	jump.eq	@.BB0_13
	shl.s	3, r3, r3
	shl.s	5, r2, r2
	ld.1	r2, r4
	shl	r4, r3, r4
	shr	r4, r3, r4
	ptr.add	r1, r2, r1
	ld	r1, r1
	sub	256, r3, r3
	shr	r1, r3, r1
	shl	r1, r3, r1
	or	r1, r4, r1
	st.1	r2, r1
.BB0_13:
	add	stack[@calldatasize], r0, r1
	add	r1, r0, stack-[1]
	add	@CPI0_4[0], r0, r1
	st.2	0, r1
	context.code_source	r1
	st.2	4, r1
	st.2	36, r0
	add	@CPI0_0[0], r0, r1
	context.gas_left	r2
	sub.s!	@CPI0_0[0], r2, r3
	add.lt	r2, r0, r1
	shl.s	192, r1, r1
	or	@CPI0_5[0], r1, r1
	add	32773, r0, r2
	near_call	r0, @__staticcall, @DEFAULT_UNWIND
	and!	1, r2, r2
	jump.eq	@.BB0_6
	add	stack-[1], r0, r5
	ld	r1, r2
	context.gas_left	r1
	sub.s!	4, r2, r3
	jump.ne	@.BB0_19
	ptr.add	stack[@ptr_return_data], r0, r1
	add	stack[@returndatasize], r0, r2
	and	31, r2, r3
	shr.s!	5, r2, r2
	jump.eq	@.BB0_26
	add	r0, r0, r4
.BB0_17:
	shl.s	5, r4, r5
	ptr.add	r1, r5, r6
	ld	r6, r6
	st.1	r5, r6
	add	1, r4, r4
	sub!	r4, r2, r5
	jump.lt	@.BB0_17
.BB0_26:
	sub!	r3, r0, r4
	jump.eq	@.BB0_25
	shl.s	3, r3, r3
	shl.s	5, r2, r2
	ld.1	r2, r4
	shl	r4, r3, r4
	shr	r4, r3, r4
	ptr.add	r1, r2, r1
	ld	r1, r1
	sub	256, r3, r3
	shr	r1, r3, r1
	shl	r1, r3, r1
	or	r1, r4, r1
	st.1	r2, r1
	jump	@.BB0_25
.BB0_1:
	add	191, r3, r1
	and	@CPI0_1[0], r1, r1
	st.1	64, r1
	and	31, r3, r1
	ptr.add	stack[@ptr_calldata], r0, r2
	shr.s!	5, r3, r4
	jump.eq	@.BB0_29
	add	r0, r0, r5
.BB0_3:
	shl.s	5, r5, r6
	ptr.add	r2, r6, r7
	ld	r7, r7
	add	160, r6, r6
	st.1	r6, r7
	add	1, r5, r5
	sub!	r5, r4, r6
	jump.lt	@.BB0_3
.BB0_29:
	sub!	r1, r0, r5
	jump.eq	@.BB0_5
	shl.s	5, r4, r4
	ptr.add	r2, r4, r2
	shl.s	3, r1, r1
	add	160, r4, r4
	ld.1	r4, r5
	shl	r5, r1, r5
	shr	r5, r1, r5
	ld	r2, r2
	sub	256, r1, r1
	shr	r2, r1, r2
	shl	r2, r1, r1
	or	r1, r5, r1
	st.1	r4, r1
.BB0_5:
	sub.s!	31, r3, r1
	jump.le	@.BB0_6
	ld.1	160, r1
	sub.s!	@CPI0_2[0], r1, r2
	jump.le	@.BB0_8
.BB0_6:
	add	r0, r0, r1
	ret.revert.to_label	r1, @DEFAULT_FAR_REVERT
.BB0_19:
	add	@CPI0_0[0], r0, r3
	sub.s!	@CPI0_0[0], r1, r4
	add.ge	r3, r0, r1
	shl.s	192, r1, r1
	shl.s	96, r5, r3
	add	r1, r3, r1
	near_call	r0, @__delegatecall, @DEFAULT_UNWIND
	ptr.add	r1, r0, stack[@ptr_return_data]
	add	r1, r0, r3
	shr.s	96, r3, r4
	and	31, r4, r3
	and	@CPI0_0[0], r4, stack[@returndatasize]
	and	@CPI0_0[0], r4, r4
	shr.s!	5, r4, r4
	jump.eq	@.BB0_27
	add	r0, r0, r5
.BB0_21:
	shl.s	5, r5, r6
	ptr.add	r1, r6, r7
	ld	r7, r7
	st.1	r6, r7
	add	1, r5, r5
	sub!	r5, r4, r6
	jump.lt	@.BB0_21
.BB0_27:
	sub!	r3, r0, r5
	jump.eq	@.BB0_23
	shl.s	3, r3, r3
	shl.s	5, r4, r4
	ld.1	r4, r5
	shl	r5, r3, r5
	shr	r5, r3, r5
	ptr.add	r1, r4, r1
	ld	r1, r1
	sub	256, r3, r3
	shr	r1, r3, r1
	shl	r1, r3, r1
	or	r1, r5, r1
	st.1	r4, r1
.BB0_23:
	and!	1, r2, r1
	jump.eq	@.BB0_24
.BB0_25:
	add	96, r0, r1
	shl	stack[@returndatasize], r1, r1
	ret.ok.to_label	r1, @DEFAULT_FAR_RETURN
.BB0_8:
	st.1	128, r1
	st.2	320, r0
	st.2	352, r1
	add	32, r0, r1
	st.2	256, r1
	add	1, r0, r1
	st.2	288, r1
	add	@CPI0_3[0], r0, r1
	ret.ok.to_label	r1, @DEFAULT_FAR_RETURN
.BB0_24:
	add	96, r0, r1
	shl	stack[@returndatasize], r1, r1
	ret.revert.to_label	r1, @DEFAULT_FAR_REVERT
.func_end0:

__staticcall:
.func_begin1:
.tmp0:
	far_call.static	r1, r2, @.BB1_2
.tmp1:
	add	1, r0, r2
	ret
.BB1_2:
.tmp2:
	add	r0, r0, r2
	ret
.func_end1:

__delegatecall:
.func_begin2:
.tmp3:
	far_call.delegate	r1, r2, @.BB2_2
.tmp4:
	add	1, r0, r2
	ret
.BB2_2:
.tmp5:
	add	r0, r0, r2
	ret
.func_end2:

	.data
	.p2align	5
calldatasize:
	.cell 0

	.p2align	5
returndatasize:
	.cell 0

	.p2align	5
ptr_calldata:
.cell	0

	.p2align	5
ptr_return_data:
.cell	0

	.note.GNU-stack
	.rodata
CPI0_0:
	.cell 4294967295
CPI0_1:
	.cell 8589934560
CPI0_2:
	.cell 1461501637330902918203684832716283019655932542975
CPI0_3:
	.cell 53919893334301279589334030174039261357415493651629346657050491355136
CPI0_4:
	.cell 22182216476136578060272566318850604970565072242024486780356928325126096266030
CPI0_5:
	.cell 904625751086426111047927909714404454142933107862120802609382293630030446592
"#;
