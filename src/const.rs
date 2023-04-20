//!
//! Vyper to zkEVM compiler constants.
//!

#![allow(dead_code)]

use lazy_static::lazy_static;
use sha3::Digest;

/// The `FREE_VAR_SPACE` offset.
pub const OFFSET_FREE_VAR_SPACE: usize = 0;

/// The `FREE_VAR_SPACE2` offset.
pub const OFFSET_FREE_VAR_SPACE2: usize =
    OFFSET_FREE_VAR_SPACE + compiler_common::BYTE_LENGTH_FIELD;

/// The non-reserved memory offset.
pub const OFFSET_NON_RESERVED: usize = OFFSET_FREE_VAR_SPACE2 + compiler_common::BYTE_LENGTH_FIELD;

/// The default label destination prefix.
pub const LABEL_DESTINATION_PREFIX: &str = "_sym_";

/// The default sequence identifier.
pub const DEFAULT_SEQUENCE_IDENTIFIER: &str = "seq";

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

/// The forwarder contract name.
pub const FORWARDER_CONTRACT_NAME: &str = "__VYPER_FORWARDER_CONTRACT";

lazy_static! {
    ///
    /// The Vyper forwarder bytecode in bytes.
    ///
    pub static ref FORWARDER_CONTRACT_BYTECODE_WORDS: Vec<[u8; compiler_common::BYTE_LENGTH_FIELD]> = {
        let metadata_hash = sha3::Keccak256::digest(FORWARDER_CONTRACT_ASSEMBLY.as_bytes()).into();
        let mut assembly =
            zkevm_assembly::Assembly::from_string(FORWARDER_CONTRACT_ASSEMBLY.to_owned(), Some(metadata_hash)).expect("Always valid");
        assembly
            .compile_to_bytecode().expect("Always valid")
    };

    ///
    /// The Vyper forwarder bytecode in words.
    ///
    pub static ref FORWARDER_CONTRACT_BYTECODE: Vec<u8> = {
        FORWARDER_CONTRACT_BYTECODE_WORDS.clone()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>()
    };

    ///
    /// The Vyper forwarder bytecode hash.
    ///
    pub static ref FORWARDER_CONTRACT_HASH: String =
        zkevm_opcode_defs::bytecode_to_code_hash(FORWARDER_CONTRACT_BYTECODE_WORDS.as_slice()).map(hex::encode)
            .expect("Always valid");
}

/// The forwarder contract assembly.
pub const FORWARDER_CONTRACT_ASSEMBLY: &str = r#"
	.text
	.file	"forwarder.sol:Test"
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
	add	64, r0, r2
	st.1	r2, r1
	ptr.add	stack[@ptr_calldata], r0, r1
	add	stack[@calldatasize], r0, r2
	and	31, r2, r3
	shr.s	5, r2, r2
	sub.s!	0, r2, r4
	jump.eq	@.BB0_28
	add	r0, r0, r4
.BB0_12:
	shl.s	5, r4, r5
	ptr.add	r1, r5, r6
	ld	r6, r6
	st.1	r5, r6
	add	1, r4, r4
	sub!	r4, r2, r5
	jump.lt	@.BB0_12
.BB0_28:
	sub.s!	0, r3, r4
	jump.eq	@.BB0_14
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
.BB0_14:
	context.ergs_left	r1
	add	r1, r0, stack-[1]
	add	@CPI0_4[0], r0, r1
	st.2	r0, r1
	context.code_source	r1
	add	4, r0, r2
	st.2	r2, r1
	add	36, r0, r1
	st.2	r1, r0
	near_call	r0, @__system_request, @DEFAULT_UNWIND
	add	r1, r0, r2
	sub.s!	4, r2, r1
	jump.ne	@.BB0_15
	ptr.add	stack[@ptr_return_data], r0, r1
	add	stack[@returndatasize], r0, r2
	and	31, r2, r3
	shr.s	5, r2, r2
	sub.s!	0, r2, r4
	jump.eq	@.BB0_26
	add	r0, r0, r4
.BB0_23:
	shl.s	5, r4, r5
	ptr.add	r1, r5, r6
	ld	r6, r6
	st.1	r5, r6
	add	1, r4, r4
	sub!	r4, r2, r5
	jump.lt	@.BB0_23
.BB0_26:
	sub.s!	0, r3, r4
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
	add	64, r0, r2
	st.1	r2, r1
	and	31, r3, r1
	ptr.add	stack[@ptr_calldata], r0, r2
	shr.s	5, r3, r4
	sub.s!	0, r4, r5
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
	sub.s!	0, r1, r5
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
	jump.gt	@.BB0_7
	add	r0, r0, r1
	near_call	r0, @__exit_revert, @DEFAULT_UNWIND
.BB0_15:
	add	stack[@calldatasize], r0, r3
	add	stack-[1], 0, r1
	near_call	r0, @__default_delegate_call, @DEFAULT_UNWIND
	ptr.add	stack[@ptr_return_data], r0, r2
	add	stack[@returndatasize], r0, r3
	and	31, r3, r4
	shr.s	5, r3, r3
	sub.s!	0, r3, r5
	jump.eq	@.BB0_27
	add	r0, r0, r5
.BB0_17:
	shl.s	5, r5, r6
	ptr.add	r2, r6, r7
	ld	r7, r7
	st.1	r6, r7
	add	1, r5, r5
	sub!	r5, r3, r6
	jump.lt	@.BB0_17
.BB0_27:
	sub.s!	0, r4, r5
	jump.eq	@.BB0_19
	shl.s	3, r4, r4
	shl.s	5, r3, r3
	ld.1	r3, r5
	shl	r5, r4, r5
	shr	r5, r4, r5
	ptr.add	r2, r3, r2
	ld	r2, r2
	sub	256, r4, r4
	shr	r2, r4, r2
	shl	r2, r4, r2
	or	r2, r5, r2
	st.1	r3, r2
.BB0_19:
	sub.s!	0, r1, r1
	jump.ne	@.BB0_25
	add	stack[@returndatasize], r0, r1
	near_call	r0, @__exit_revert, @DEFAULT_UNWIND
.BB0_25:
	add	stack[@returndatasize], r0, r2
	add	r0, r0, r1
	add	r0, r0, r3
	near_call	r0, @__exit_return, @DEFAULT_UNWIND
.BB0_7:
	add	160, r0, r1
	ld.1	r1, r1
	sub.s!	@CPI0_2[0], r1, r2
	jump.lt	@.BB0_9
	add	r0, r0, r1
	near_call	r0, @__exit_revert, @DEFAULT_UNWIND
.BB0_9:
	add	128, r0, r2
	st.1	r2, r1
	add	320, r0, r3
	st.2	r3, r0
	add	352, r0, r3
	st.2	r3, r1
	add	32, r0, r3
	add	256, r0, r1
	st.2	r1, r3
	add	1, r0, r3
	add	288, r0, r4
	st.2	r4, r3
	add	@CPI0_3[0], r0, r3
	near_call	r0, @__exit_return, @DEFAULT_UNWIND
.func_end0:

__default_delegate_call:
.func_begin1:
	add	@CPI1_0[0], r0, r4
	sub.s!	@CPI1_0[0], r1, r5
	add.ge	r4, r0, r1
	shl.s	192, r1, r1
	shl.s	96, r3, r3
	add	r3, r1, r1
	near_call	r0, @__delegatecall, @DEFAULT_UNWIND
	add	r1, r0, r3
	shr.s	96, r3, r3
	and	@CPI1_0[0], r3, stack[@returndatasize]
	ptr.add	r1, r0, stack[@ptr_return_data]
	and	1, r2, r1
	ret
.func_end1:

__system_request:
.func_begin2:
	add	@CPI2_0[0], r0, r1
	context.ergs_left	r2
	sub.s!	@CPI2_0[0], r2, r3
	add.lt	r2, r0, r1
	shl.s	192, r1, r1
	or	@CPI2_1[0], r1, r1
	add	32773, r0, r2
	near_call	r0, @__staticcall, @DEFAULT_UNWIND
	and!	1, r2, r2
	jump.eq	@.BB2_2
	ld	r1, r1
	ret
.BB2_2:
	add	r0, r0, r1
	near_call	r0, @__exit_revert, @DEFAULT_UNWIND
.func_end2:

__exit_return:
.func_begin3:
	shl.s	64, r1, r1
	shl.s	96, r2, r2
	add	r2, r1, r1
	add	r1, r3, r1
	ret.ok.to_label	r1, @DEFAULT_FAR_RETURN
.func_end3:

__exit_revert:
.func_begin4:
	shl.s	96, r1, r1
	ret.revert.to_label	r1, @DEFAULT_FAR_REVERT
.func_end4:

__staticcall:
.func_begin5:
.tmp0:
	far_call.static	r1, r2, @.BB5_2
.tmp1:
	add	1, r0, r2
	ret
.BB5_2:
.tmp2:
	add	r0, r0, r2
	ret
.func_end5:

__delegatecall:
.func_begin6:
.tmp3:
	far_call.delegate	r1, r2, @.BB6_2
.tmp4:
	add	1, r0, r2
	ret
.BB6_2:
.tmp5:
	add	r0, r0, r2
	ret
.func_end6:

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
CPI1_0:
CPI2_0:
	.cell 4294967295
CPI0_1:
	.cell 8589934560
CPI0_2:
	.cell 1461501637330902918203684832716283019655932542976
CPI0_3:
	.cell 53919893334301279589334030174039261347274288845081144962207220498432
CPI0_4:
	.cell 22182216476136578060272566318850604970565072242024486780356928325126096266030
CPI2_1:
	.cell 904625751086426111047927909714404454142933107862120802609382293630030446592
"#;
