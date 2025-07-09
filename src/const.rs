//!
//! Vyper compiler constants.
//!

#![allow(dead_code)]

use std::collections::BTreeMap;

use lazy_static::lazy_static;

/// The `vyper` compiler production name.
pub static VYPER_PRODUCTION_NAME: &str = "vyper";

/// The default executable name.
pub static DEFAULT_EXECUTABLE_NAME: &str = "zkvyper";

///
/// The compiler version default function.
///
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// The worker thread stack size.
pub const WORKER_THREAD_STACK_SIZE: usize = 64 * 1024 * 1024;

/// The `FREE_VAR_SPACE` offset.
pub const OFFSET_FREE_VAR_SPACE: usize = 0;

/// The `FREE_VAR_SPACE2` offset.
pub const OFFSET_FREE_VAR_SPACE2: usize =
    OFFSET_FREE_VAR_SPACE + era_compiler_common::BYTE_LENGTH_FIELD;

/// The non-reserved memory offset.
pub const OFFSET_NON_RESERVED: usize =
    OFFSET_FREE_VAR_SPACE2 + era_compiler_common::BYTE_LENGTH_FIELD;

/// The constructor function name.
pub const FUNCTION_NAME_CONSTRUCTOR: &str = "__init__";

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

/// The forbidden function `raw_create` introduced at 0.4.2.
pub const FORBIDDEN_FUNCTION_NAME_RAW_CREATE: &str = "raw_create";

lazy_static! {
    /// Vyper minimal proxy bytecode in bytes.
    pub static ref MINIMAL_PROXY_BUILD: era_compiler_llvm_context::EraVMBuild = {
        let target_machine = era_compiler_llvm_context::TargetMachine::new(era_compiler_common::Target::EraVM, &era_compiler_llvm_context::OptimizerSettings::cycles(), &[])
                .expect("Minimal proxy target machine initialization error");

        let assembly_buffer = era_compiler_llvm_context::eravm_assemble(&target_machine, MINIMAL_PROXY_CONTRACT_NAME, MINIMAL_PROXY_CONTRACT_ASSEMBLY, None)
                .expect("Minimal proxy assembling error");
        let mut build = era_compiler_llvm_context::eravm_build(assembly_buffer, None, None, Some(MINIMAL_PROXY_CONTRACT_ASSEMBLY.to_owned()))
                .expect("Minimal proxy building error");

        let bytecode_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(build.bytecode.as_slice(), MINIMAL_PROXY_CONTRACT_NAME, false);
        let (bytecode_buffer_linked, object_format) = era_compiler_llvm_context::eravm_link(bytecode_buffer, &BTreeMap::new(), &BTreeMap::new()).expect("Minimal proxy linking error");
        assert_eq!(object_format, era_compiler_common::ObjectFormat::Raw, "Minimal proxy object format error");

        build.bytecode = bytecode_buffer_linked.as_slice().to_vec();
        build.bytecode_hash = Some(era_compiler_llvm_context::eravm_hash(&bytecode_buffer_linked).expect("Minimal proxy hashing error"));
        build
    };
}

/// Minimal proxy contract assembly.
pub const MINIMAL_PROXY_CONTRACT_ASSEMBLY: &str = r#"
        .text
        .file   "MinimalProxy.sol:MinimalProxy"
        .globl  __entry
__entry:
.func_begin0:
        incsp   1
        shr.s   96, r1, r5
        and     31, r5, r4
        and     code[@CPI0_0], r5, r3
        and     code[@CPI0_1], r5, r8
        and!    1, r2, r0
        jump.ne @.BB0_1
        add     128, r0, r2
        stm.h   64, r2
        sub!    r3, r0, r0
        jump.eq @.BB0_12
        addp    r1, r0, r2
        add     r0, r0, r5
.BB0_11:
        ldpi    r2, r6, r2
        stmi.h  r5, r6, r5
        sub!    r5, r3, r0
        jump.ne @.BB0_11
.BB0_12:
        add     r8, r0, stack-[1]
        sub!    r4, r0, r0
        jump.eq @.BB0_14
        addp    r1, r3, r1
        shl.s   3, r4, r2
        ldm.h   r3, r4
        shl     r4, r2, r4
        shr     r4, r2, r4
        ldp     r1, r1
        sub     256, r2, r2
        shr     r1, r2, r1
        shl     r1, r2, r1
        or      r1, r4, r1
        stm.h   r3, r1
.BB0_14:
        add     code[@CPI0_5], r0, r1
        stm.ah  0, r1
        code    r1
        stm.ah  4, r1
        stm.ah  36, r0
        ergs    r1
        sub.s!  code[@CPI0_1], r1, r0
        add.ge  code[@CPI0_1], r0, r1
        shl.s   192, r1, r1
        or      code[@CPI0_6], r1, r1
        add     32773, r0, r2
        call    r0, @__staticcall, @DEFAULT_UNWIND
        and!    1, r2, r0
        jump.eq @.BB0_23
        ldp     r1, r2
        add     stack-[1], r0, r1
        shl.s   96, r1, r1
        ergs    r3
        sub.s!  code[@CPI0_1], r3, r0
        add.ge  code[@CPI0_1], r0, r3
        shl.s   192, r3, r3
        or      r3, r1, r1
        call    r0, @__delegatecall, @DEFAULT_UNWIND
        shr.s   96, r1, r5
        and     31, r5, r4
        and!    code[@CPI0_0], r5, r3
        jump.eq @.BB0_18
        addp    r1, r0, r6
        add     r0, r0, r7
.BB0_17:
        ldpi    r6, r8, r6
        stmi.h  r7, r8, r7
        sub!    r7, r3, r0
        jump.ne @.BB0_17
.BB0_18:
        and     code[@CPI0_1], r5, r5
        sub!    r4, r0, r0
        jump.eq @.BB0_20
        addp    r1, r3, r1
        shl.s   3, r4, r4
        ldm.h   r3, r6
        shl     r6, r4, r6
        shr     r6, r4, r6
        ldp     r1, r1
        sub     256, r4, r4
        shr     r1, r4, r1
        shl     r1, r4, r1
        or      r1, r6, r1
        stm.h   r3, r1
.BB0_20:
        shl.s   96, r5, r1
        and!    1, r2, r0
        jump.eq @.BB0_24
        retl    @DEFAULT_FAR_RETURN
.BB0_1:
        add     31, r8, r2
        and     code[@CPI0_2], r2, r2
        add     160, r2, r2
        stm.h   64, r2
        add     160, r3, r2
        sub!    r3, r0, r0
        jump.eq @.BB0_4
        add     160, r0, r5
        addp    r1, r0, r6
.BB0_3:
        ldpi    r6, r7, r6
        stmi.h  r5, r7, r5
        sub!    r5, r2, r0
        jump.ne @.BB0_3
.BB0_4:
        sub!    r4, r0, r0
        jump.eq @.BB0_6
        addp    r1, r3, r1
        shl.s   3, r4, r3
        ldm.h   r2, r4
        shl     r4, r3, r4
        shr     r4, r3, r4
        ldp     r1, r1
        sub     256, r3, r3
        shr     r1, r3, r1
        shl     r1, r3, r1
        or      r1, r4, r1
        stm.h   r2, r1
.BB0_6:
        sub.s!  31, r8, r0
        jump.le @.BB0_22
        ldm.h   160, r1
        sub.s!  code[@CPI0_3], r1, r0
        jump.le @.BB0_8
.BB0_22:
        add     r0, r0, r1
        revl    @DEFAULT_FAR_REVERT
.BB0_23:
        rev
.BB0_24:
        revl    @DEFAULT_FAR_REVERT
.BB0_8:
        stm.h   128, r1
        stm.ah  320, r0
        stm.ah  352, r1
        add     32, r0, r1
        stm.ah  256, r1
        add     1, r0, r1
        stm.ah  288, r1
        add     code[@CPI0_4], r0, r1
        retl    @DEFAULT_FAR_RETURN
.func_end0:

__cxa_throw:
.func_begin1:
        rev
.func_end1:

__staticcall:
.func_begin2:
.tmp0:
        callf.st        r1, r2, @.BB2_2
.tmp1:
        add     1, r0, r2
        ret
.BB2_2:
.tmp2:
        add     r0, r0, r2
        ret
.func_end2:

__delegatecall:
.func_begin3:
.tmp3:
        calld   r1, r2, @.BB3_2
.tmp4:
        add     1, r0, r2
        ret
.BB3_2:
.tmp5:
        add     r0, r0, r2
        ret
.func_end3:

        .rodata
CPI0_0:
        .cell   4294967264
CPI0_1:
        .cell   4294967295
CPI0_2:
        .cell   8589934560
CPI0_3:
        .cell   1461501637330902918203684832716283019655932542975
CPI0_4:
        .cell   53919893334301279589334030174039261357415493651629346657050491355136
CPI0_5:
        .cell   22182216476136578060272566318850604970565072242024486780356928325126096266030
CPI0_6:
        .cell   904625751086426111047927909714404454142933107862120802609382293630030446592
        .text
DEFAULT_UNWIND:
        pncl    @DEFAULT_UNWIND
DEFAULT_FAR_RETURN:
        retl    @DEFAULT_FAR_RETURN
DEFAULT_FAR_REVERT:
        revl    @DEFAULT_FAR_REVERT
"#;
