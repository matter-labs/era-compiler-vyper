//!
//! The LLL IR instruction.
//!

pub mod assert;
pub mod clamp;
pub mod create;
pub mod deploy;
pub mod exit_to;
pub mod goto;
pub mod r#if;
pub mod immutable;
pub mod label;
pub mod offset;
pub mod repeat;
pub mod r#return;
pub mod revert;
pub mod seq;
pub mod set;
pub mod with;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use inkwell::values::BasicValue;

use crate::project::contract::vyper::expression::Expression;

use self::assert::Assert;
use self::deploy::Deploy;
use self::exit_to::ExitTo;
use self::goto::Goto;
use self::label::Label;
use self::r#if::If;
use self::r#return::Return;
use self::repeat::Repeat;
use self::revert::Revert;
use self::seq::Seq;
use self::set::Set;
use self::with::With;

///
/// The LLL IR instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    /// The LLL IR `with` expression.
    With(With),
    /// The LLL IR `set` expression.
    Set(Set),
    /// The LLL IR `seq` expression.
    Seq(Seq),
    /// The LLL IR `if` statement.
    If(If),
    /// The LLL IR `repeat` statement.
    Repeat(Repeat),

    /// The LLL IR `goto` instruction.
    GoTo(Goto),
    /// The LLL IR `exit_to` instruction.
    Exit_To(ExitTo),
    /// The LLL IR `jump` instruction.
    Jump([Box<Expression>; 1]),
    /// The LLL IR `label` instruction.
    Label(Label),
    /// The LLL IR `cleanup_repeat` instruction.
    Cleanup_Repeat,
    /// The LLL IR `break` instruction.
    Break,
    /// The LLL IR `continue` instruction.
    Continue,
    /// The LLL IR `pass` instruction.
    Pass,
    /// The LLL IR `deploy` instruction.
    Deploy(Deploy),
    /// The LLL IR `symbol` instruction.
    Symbol([Box<Expression>; 1]),
    /// The LLL IR `unique_symbol` instruction.
    Unique_Symbol([Box<Expression>; 1]),

    /// The LLL IR pseudo opcode.
    UCLAMP([Box<Expression>; 3]),
    /// The LLL IR pseudo opcode.
    CLAMP([Box<Expression>; 3]),
    /// The LLL IR pseudo opcode.
    UCLAMPLT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    UCLAMPLE([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    UCLAMPGT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    UCLAMPGE([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    CLAMPLT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    CLAMPLE([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    CLAMPGT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    CLAMPGE([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    CLAMP_NONZERO([Box<Expression>; 1]),

    /// The LLL IR pseudo opcode.
    CEIL32([Box<Expression>; 1]),
    /// The LLL IR pseudo opcode.
    SELECT([Box<Expression>; 3]),

    /// The LLL IR pseudo opcode.
    Assert(Assert),
    /// The LLL IR pseudo opcode.
    Assert_Unreachable(Assert),

    /// The LLL IR `var_list` instruction.
    Var_List(Vec<Expression>),

    /// The LLL IR EVM opcode.
    POP([Box<Expression>; 1]),

    /// The LLL IR EVM opcode.
    ADD([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SUB([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    MUL([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    DIV([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    MOD([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SDIV([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SMOD([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    LT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    LE([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    GT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    GE([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    EQ([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    NE([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    ISZERO([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    SLT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    SLE([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SGT([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    SGE([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    OR([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    XOR([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    NOT([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    AND([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SHL([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SHR([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SAR([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    BYTE([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    ADDMOD([Box<Expression>; 3]),
    /// The LLL IR EVM opcode.
    MULMOD([Box<Expression>; 3]),
    /// The LLL IR EVM opcode.
    EXP([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    SIGNEXTEND([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    SHA3([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    KECCAK256([Box<Expression>; 2]),
    /// The LLL IR pseudo opcode.
    SHA3_32([Box<Expression>; 1]),
    /// The LLL IR pseudo opcode.
    SHA3_64([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    MLOAD([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    MSTORE([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    MSTORE8([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    SLOAD([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    SSTORE([Box<Expression>; 2]),

    /// The LLL IR EVM opcode.
    ILOAD([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    ISTORE([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    DLOAD([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    DLOADBYTES([Box<Expression>; 3]),

    /// The LLL IR EVM opcode.
    CALLDATALOAD([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    CALLDATASIZE,
    /// The LLL IR EVM opcode.
    CALLDATACOPY([Box<Expression>; 3]),
    /// The LLL IR EVM opcode.
    CODESIZE,
    /// The LLL IR EVM opcode.
    CODECOPY([Box<Expression>; 3]),
    /// The LLL IR EVM opcode.
    EXTCODESIZE([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    EXTCODEHASH([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    RETURNDATASIZE,
    /// The LLL IR EVM opcode.
    RETURNDATACOPY([Box<Expression>; 3]),

    /// The LLL IR EVM opcode.
    RETURN(Return),
    /// The LLL IR EVM opcode.
    REVERT(Revert),
    /// The LLL IR EVM opcode.
    STOP,
    /// The LLL IR EVM opcode.
    INVALID,

    /// The LLL IR EVM opcode.
    LOG0([Box<Expression>; 2]),
    /// The LLL IR EVM opcode.
    LOG1([Box<Expression>; 3]),
    /// The LLL IR EVM opcode.
    LOG2([Box<Expression>; 4]),
    /// The LLL IR EVM opcode.
    LOG3([Box<Expression>; 5]),
    /// The LLL IR EVM opcode.
    LOG4([Box<Expression>; 6]),

    /// The LLL IR EVM opcode.
    CALL([Box<Expression>; 7]),
    /// The LLL IR EVM opcode.
    STATICCALL([Box<Expression>; 6]),
    /// The LLL IR EVM opcode.
    DELEGATECALL([Box<Expression>; 6]),

    /// The LLL IR EVM opcode.
    CREATE([Box<Expression>; 3]),
    /// The LLL IR EVM opcode.
    CREATE2([Box<Expression>; 4]),

    /// The LLL IR EVM opcode.
    ADDRESS,
    /// The LLL IR EVM opcode.
    CALLER,

    /// The LLL IR EVM opcode.
    CALLVALUE,
    /// The LLL IR EVM opcode.
    GAS,
    /// The LLL IR EVM opcode.
    BALANCE([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    SELFBALANCE,

    /// The LLL IR EVM opcode.
    GASLIMIT,
    /// The LLL IR EVM opcode.
    GASPRICE,
    /// The LLL IR EVM opcode.
    ORIGIN,
    /// The LLL IR EVM opcode.
    CHAINID,
    /// The LLL IR EVM opcode.
    NUMBER,
    /// The LLL IR EVM opcode.
    TIMESTAMP,
    /// The LLL IR EVM opcode.
    BLOCKHASH([Box<Expression>; 1]),
    /// The LLL IR EVM opcode.
    DIFFICULTY,
    /// The LLL IR EVM opcode.
    COINBASE,
    /// The LLL IR EVM opcode.
    BASEFEE,
    /// The LLL IR EVM opcode.
    MSIZE,

    /// The LLL IR EVM opcode.
    CALLCODE([Box<Expression>; 7]),
    /// The LLL IR EVM opcode.
    PC,
    /// The LLL IR EVM opcode.
    EXTCODECOPY([Box<Expression>; 4]),
    /// The LLL IR EVM opcode.
    SELFDESTRUCT([Box<Expression>; 1]),

    /// The LLL unknown trap.
    Unknown(serde_json::Value),
}

impl Instruction {
    ///
    /// Translates the specified number of arguments.
    ///
    fn translate_arguments_llvm<'ctx, D, const N: usize>(
        arguments: [Box<Expression>; N],
        context: &mut compiler_llvm_context::EraVMContext<'ctx, D>,
    ) -> anyhow::Result<[inkwell::values::BasicValueEnum<'ctx>; N]>
    where
        D: compiler_llvm_context::EraVMDependency + Clone,
    {
        let debug_string = format!("`{arguments:?}`");

        let mut values = Vec::with_capacity(N);
        for (index, expression) in arguments.into_iter().enumerate().rev() {
            values.push(expression.into_llvm_value(context)?.ok_or_else(|| {
                anyhow::anyhow!(
                    "Expression #{} of the instruction `{}` has zero valency",
                    index,
                    debug_string
                )
            })?);
        }
        values.reverse();

        if values.len() != N {
            anyhow::bail!(
                "Expected {} arguments, found only {}: `{:?}`",
                N,
                values.len(),
                values
            );
        }

        Ok(values.try_into().expect("Always valid"))
    }

    ///
    /// Extracts the functions from the deploy or runtime code.
    ///
    pub fn extract_functions(&mut self) -> anyhow::Result<BTreeMap<String, Expression>> {
        match self {
            Self::Seq(inner) => inner.extract_functions(),
            Self::With(inner) => inner.extract_functions(),
            Self::If(inner) => inner.extract_functions(),
            Self::Repeat(inner) => inner.extract_functions(),
            Self::Label(inner) => inner.extract_functions(),
            _ => Ok(BTreeMap::new()),
        }
    }

    ///
    /// Whether the instruction is a function entry block.
    ///
    pub fn is_function(&self) -> anyhow::Result<bool> {
        match self {
            Self::Seq(sequence) => sequence.is_function(),
            _ => Ok(false),
        }
    }

    ///
    /// Returns the function name.
    ///
    pub fn function_name(&self) -> anyhow::Result<String> {
        match self {
            Self::Seq(inner) => inner.function_name(),
            expression => anyhow::bail!("Expected a function sequence, found `{:?}`", expression),
        }
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<'ctx, D>(
        self,
        context: &mut compiler_llvm_context::EraVMContext<'ctx, D>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
    where
        D: compiler_llvm_context::EraVMDependency + Clone,
    {
        match self {
            Self::With(inner) => inner.into_llvm_value(context),
            Self::Set(inner) => inner.into_llvm_value(context).map(|_| None),
            Self::Seq(inner) => inner.into_llvm_value(context),
            Self::If(inner) => inner.into_llvm_value(context),
            Self::Repeat(inner) => inner.into_llvm_value(context).map(|_| None),

            Self::GoTo(inner) => inner.into_llvm_value(context),
            Self::Exit_To(inner) => inner.into_llvm_value(context).map(|_| None),
            Self::Jump(arguments) => {
                let _arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                let block = context.current_function().borrow().return_block();
                context.build_unconditional_branch(block);
                Ok(None)
            }
            Self::Label(inner) => inner.into_llvm_value(context).map(|_| None),
            Self::Cleanup_Repeat => Ok(None),
            Self::Break => {
                let block = context.r#loop().join_block;
                context.build_unconditional_branch(block);
                Ok(None)
            }
            Self::Continue => {
                let block = context.r#loop().continue_block;
                context.build_unconditional_branch(block);
                Ok(None)
            }
            Self::Pass => Ok(None),
            Self::Deploy(_inner) => Ok(None),
            Self::Symbol(_inner) => Ok(None),
            Self::Unique_Symbol(_inner) => Ok(None),

            Self::UCLAMP(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                clamp::ordinary(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                    false,
                )
                .map(Some)
            }
            Self::CLAMP(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                clamp::ordinary(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                    true,
                )
                .map(Some)
            }
            Self::UCLAMPLT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULT,
                )
                .map(Some)
            }
            Self::UCLAMPLE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULE,
                )
                .map(Some)
            }
            Self::UCLAMPGT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGT,
                )
                .map(Some)
            }
            Self::UCLAMPGE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGE,
                )
                .map(Some)
            }
            Self::CLAMPLT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLT,
                )
                .map(Some)
            }
            Self::CLAMPLE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLE,
                )
                .map(Some)
            }
            Self::CLAMPGT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGT,
                )
                .map(Some)
            }
            Self::CLAMPGE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGE,
                )
                .map(Some)
            }
            Self::CLAMP_NONZERO(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                clamp::with_predicate(
                    context,
                    arguments[0].into_int_value(),
                    context.field_const(0),
                    inkwell::IntPredicate::NE,
                )
                .map(Some)
            }

            Self::CEIL32(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                offset::ceil_32(context, arguments[0].into_int_value()).map(Some)
            }
            Self::SELECT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                let condition = context.builder().build_int_compare(
                    inkwell::IntPredicate::NE,
                    arguments[0].into_int_value(),
                    context.field_const(0),
                    "select_condition",
                );
                Ok(Some(context.builder().build_select(
                    condition,
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                    "select",
                )))
            }

            Self::Assert(inner) => inner.into_llvm_value(context).map(|_| None),
            Self::Assert_Unreachable(inner) => inner.into_llvm_value(context).map(|_| None),

            Self::Var_List(_inner) => Ok(None),

            Self::POP(arguments) => {
                let _arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                Ok(None)
            }

            Self::ADD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::addition(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SUB(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::subtraction(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::MUL(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::multiplication(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::DIV(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::division(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::MOD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::remainder(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SDIV(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::division_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SMOD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_arithmetic::remainder_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Self::LT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULT,
                )
                .map(Some)
            }
            Self::LE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULE,
                )
                .map(Some)
            }
            Self::GT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGT,
                )
                .map(Some)
            }
            Self::GE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGE,
                )
                .map(Some)
            }
            Self::EQ(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            Self::NE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::NE,
                )
                .map(Some)
            }
            Self::ISZERO(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    context.field_const(0),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            Self::SLT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLT,
                )
                .map(Some)
            }
            Self::SLE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLE,
                )
                .map(Some)
            }
            Self::SGT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGT,
                )
                .map(Some)
            }
            Self::SGE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGE,
                )
                .map(Some)
            }

            Self::OR(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::or(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::XOR(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::NOT(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    context.field_type().const_all_ones(),
                )
                .map(Some)
            }
            Self::AND(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::and(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SHL(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::shift_left(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SHR(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::shift_right(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SAR(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::shift_right_arithmetic(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::BYTE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_bitwise::byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Self::ADDMOD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                compiler_llvm_context::eravm_evm_math::add_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            Self::MULMOD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                compiler_llvm_context::eravm_evm_math::mul_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            Self::EXP(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_math::exponent(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SIGNEXTEND(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_math::sign_extend(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Self::SHA3(arguments) | Self::KECCAK256(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_crypto::sha3(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Self::SHA3_32(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;

                let pointer_one = compiler_llvm_context::EraVMPointer::new_with_offset(
                    context,
                    compiler_llvm_context::EraVMAddressSpace::Heap,
                    context.field_type(),
                    context.field_const(crate::r#const::OFFSET_FREE_VAR_SPACE as u64),
                    "sha3_pointer_one",
                );
                context.build_store(pointer_one, arguments[0]);

                compiler_llvm_context::eravm_evm_crypto::sha3(
                    context,
                    context.field_const(crate::r#const::OFFSET_FREE_VAR_SPACE as u64),
                    context.field_const(compiler_common::BYTE_LENGTH_FIELD as u64),
                )
                .map(Some)
            }
            Self::SHA3_64(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;

                let pointer_one = compiler_llvm_context::EraVMPointer::new_with_offset(
                    context,
                    compiler_llvm_context::EraVMAddressSpace::Heap,
                    context.field_type(),
                    context.field_const(crate::r#const::OFFSET_FREE_VAR_SPACE as u64),
                    "sha3_pointer_one",
                );
                context.build_store(pointer_one, arguments[0]);
                let pointer_two = compiler_llvm_context::EraVMPointer::new_with_offset(
                    context,
                    compiler_llvm_context::EraVMAddressSpace::Heap,
                    context.field_type(),
                    context.field_const(crate::r#const::OFFSET_FREE_VAR_SPACE2 as u64),
                    "sha3_pointer_two",
                );
                context.build_store(pointer_two, arguments[1]);

                compiler_llvm_context::eravm_evm_crypto::sha3(
                    context,
                    context.field_const(crate::r#const::OFFSET_FREE_VAR_SPACE as u64),
                    context.field_const((compiler_common::BYTE_LENGTH_FIELD * 2) as u64),
                )
                .map(Some)
            }

            Self::MLOAD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_memory::load(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Self::MSTORE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_memory::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Self::MSTORE8(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_memory::store_byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }

            Self::SLOAD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_storage::load(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Self::SSTORE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_storage::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }

            Self::ILOAD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_immutable::load(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Self::ISTORE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_immutable::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }

            Self::CALLDATALOAD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;

                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::EraVMCodeType::Deploy => {
                        Ok(Some(context.field_const(0).as_basic_value_enum()))
                    }
                    compiler_llvm_context::EraVMCodeType::Runtime => {
                        compiler_llvm_context::eravm_evm_calldata::load(
                            context,
                            arguments[0].into_int_value(),
                        )
                        .map(Some)
                    }
                }
            }
            Self::CALLDATASIZE => {
                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::EraVMCodeType::Deploy => {
                        Ok(Some(context.field_const(0).as_basic_value_enum()))
                    }
                    compiler_llvm_context::EraVMCodeType::Runtime => {
                        compiler_llvm_context::eravm_evm_calldata::size(context).map(Some)
                    }
                }
            }
            Self::CALLDATACOPY(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;

                let source_offset = match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::EraVMCodeType::Deploy => {
                        compiler_llvm_context::eravm_evm_calldata::size(context)?
                    }
                    compiler_llvm_context::EraVMCodeType::Runtime => {
                        arguments[1].into_int_value().as_basic_value_enum()
                    }
                }
                .into_int_value();

                compiler_llvm_context::eravm_evm_calldata::copy(
                    context,
                    arguments[0].into_int_value(),
                    source_offset,
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            Self::DLOAD(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;

                match context.code_type() {
                    None => {
                        anyhow::bail!(
                            "Immutables are not available if the contract part is undefined"
                        );
                    }
                    Some(compiler_llvm_context::EraVMCodeType::Deploy) => {
                        compiler_llvm_context::eravm_evm_calldata::load(
                            context,
                            arguments[0].into_int_value(),
                        )
                    }
                    Some(compiler_llvm_context::EraVMCodeType::Runtime) => {
                        compiler_llvm_context::eravm_evm_immutable::load(
                            context,
                            arguments[0].into_int_value(),
                        )
                    }
                }
                .map(Some)
            }
            Self::DLOADBYTES(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;

                match context.code_type() {
                    None => {
                        anyhow::bail!(
                            "Immutables are not available if the contract part is undefined"
                        );
                    }
                    Some(compiler_llvm_context::EraVMCodeType::Deploy) => {
                        compiler_llvm_context::eravm_evm_calldata::copy(
                            context,
                            arguments[0].into_int_value(),
                            arguments[1].into_int_value(),
                            arguments[2].into_int_value(),
                        )
                    }
                    Some(compiler_llvm_context::EraVMCodeType::Runtime) => immutable::load_bytes(
                        context,
                        arguments[0].into_int_value(),
                        arguments[1].into_int_value(),
                        arguments[2].into_int_value(),
                    ),
                }
                .map(|_| None)
            }
            Self::CODESIZE => {
                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::EraVMCodeType::Deploy => {
                        compiler_llvm_context::eravm_evm_calldata::size(context).map(Some)
                    }
                    compiler_llvm_context::EraVMCodeType::Runtime => {
                        let code_source =
                            compiler_llvm_context::eravm_general::code_source(context)?;
                        compiler_llvm_context::eravm_evm_ext_code::size(
                            context,
                            code_source.into_int_value(),
                        )
                        .map(Some)
                    }
                }
            }
            Self::CODECOPY(arguments) => {
                if let compiler_llvm_context::EraVMCodeType::Runtime = context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    anyhow::bail!(
                        "The `CODECOPY` instruction is not supported in the runtime code",
                    );
                }

                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                compiler_llvm_context::eravm_evm_calldata::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            Self::RETURNDATASIZE => {
                compiler_llvm_context::eravm_evm_return_data::size(context).map(Some)
            }
            Self::RETURNDATACOPY(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                compiler_llvm_context::eravm_evm_return_data::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            Self::EXTCODESIZE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_ext_code::size(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }
            Self::EXTCODEHASH(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                compiler_llvm_context::eravm_evm_ext_code::hash(
                    context,
                    arguments[0].into_int_value(),
                )
                .map(Some)
            }

            Self::RETURN(inner) => inner.into_llvm_value(context).map(|_| None),
            Self::REVERT(inner) => inner.into_llvm_value(context).map(|_| None),
            Self::STOP => compiler_llvm_context::eravm_evm_return::stop(context).map(|_| None),
            Self::INVALID => {
                compiler_llvm_context::eravm_evm_return::invalid(context).map(|_| None)
            }

            Self::LOG0(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 2>(arguments, context)?;
                compiler_llvm_context::eravm_evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    vec![],
                )
                .map(|_| None)
            }
            Self::LOG1(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;
                compiler_llvm_context::eravm_evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            Self::LOG2(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 4>(arguments, context)?;
                compiler_llvm_context::eravm_evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            Self::LOG3(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 5>(arguments, context)?;
                compiler_llvm_context::eravm_evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            Self::LOG4(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 6>(arguments, context)?;
                compiler_llvm_context::eravm_evm_event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }

            Self::CALL(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 7>(arguments, context)?;

                let gas = arguments[0].into_int_value();
                let address = arguments[1].into_int_value();
                let value = arguments[2].into_int_value();
                let input_offset = arguments[3].into_int_value();
                let input_size = arguments[4].into_int_value();
                let output_offset = arguments[5].into_int_value();
                let output_size = arguments[6].into_int_value();

                compiler_llvm_context::eravm_evm_call::default(
                    context,
                    context.llvm_runtime().far_call,
                    gas,
                    address,
                    Some(value),
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                    vec![],
                )
                .map(Some)
            }
            Self::STATICCALL(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 6>(arguments, context)?;

                let gas = arguments[0].into_int_value();
                let address = arguments[1].into_int_value();
                let input_offset = arguments[2].into_int_value();
                let input_size = arguments[3].into_int_value();
                let output_offset = arguments[4].into_int_value();
                let output_size = arguments[5].into_int_value();

                compiler_llvm_context::eravm_evm_call::default(
                    context,
                    context.llvm_runtime().static_call,
                    gas,
                    address,
                    None,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                    vec![],
                )
                .map(Some)
            }
            Self::DELEGATECALL(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 6>(arguments, context)?;

                let gas = arguments[0].into_int_value();
                let address = arguments[1].into_int_value();
                let input_offset = arguments[2].into_int_value();
                let input_size = arguments[3].into_int_value();
                let output_offset = arguments[4].into_int_value();
                let output_size = arguments[5].into_int_value();

                compiler_llvm_context::eravm_evm_call::default(
                    context,
                    context.llvm_runtime().delegate_call,
                    gas,
                    address,
                    None,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                    vec![],
                )
                .map(Some)
            }

            Self::CREATE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 3>(arguments, context)?;

                create::create(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    None,
                )
                .map(Some)
            }
            Self::CREATE2(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 4>(arguments, context)?;

                create::create(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    Some(arguments[3].into_int_value()),
                )
                .map(Some)
            }

            Self::ADDRESS => Ok(context.build_call(context.intrinsics().address, &[], "address")),
            Self::CALLER => Ok(context.build_call(context.intrinsics().caller, &[], "caller")),

            Self::CALLVALUE => compiler_llvm_context::eravm_evm_ether_gas::value(context).map(Some),
            Self::GAS => compiler_llvm_context::eravm_evm_ether_gas::gas(context).map(Some),
            Self::BALANCE(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;

                let address = arguments[0].into_int_value();
                compiler_llvm_context::eravm_evm_ether_gas::balance(context, address).map(Some)
            }
            Self::SELFBALANCE => {
                let address = context
                    .build_call(context.intrinsics().address, &[], "self_balance_address")
                    .expect("Always exists")
                    .into_int_value();

                compiler_llvm_context::eravm_evm_ether_gas::balance(context, address).map(Some)
            }

            Self::GASLIMIT => {
                compiler_llvm_context::eravm_evm_contract_context::gas_limit(context).map(Some)
            }
            Self::GASPRICE => {
                compiler_llvm_context::eravm_evm_contract_context::gas_price(context).map(Some)
            }
            Self::ORIGIN => {
                compiler_llvm_context::eravm_evm_contract_context::origin(context).map(Some)
            }
            Self::CHAINID => {
                compiler_llvm_context::eravm_evm_contract_context::chain_id(context).map(Some)
            }
            Self::NUMBER => {
                compiler_llvm_context::eravm_evm_contract_context::block_number(context).map(Some)
            }
            Self::TIMESTAMP => {
                compiler_llvm_context::eravm_evm_contract_context::block_timestamp(context)
                    .map(Some)
            }
            Self::BLOCKHASH(arguments) => {
                let arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                let index = arguments[0].into_int_value();

                compiler_llvm_context::eravm_evm_contract_context::block_hash(context, index)
                    .map(Some)
            }
            Self::DIFFICULTY => {
                compiler_llvm_context::eravm_evm_contract_context::difficulty(context).map(Some)
            }
            Self::COINBASE => {
                compiler_llvm_context::eravm_evm_contract_context::coinbase(context).map(Some)
            }
            Self::BASEFEE => {
                compiler_llvm_context::eravm_evm_contract_context::basefee(context).map(Some)
            }
            Self::MSIZE => {
                compiler_llvm_context::eravm_evm_contract_context::msize(context).map(Some)
            }

            Self::CALLCODE(arguments) => {
                let _arguments = Self::translate_arguments_llvm::<D, 7>(arguments, context)?;
                anyhow::bail!("The `CALLCODE` instruction is not supported")
            }
            Self::PC => anyhow::bail!("The `PC` instruction is not supported"),
            Self::EXTCODECOPY(arguments) => {
                let _arguments = Self::translate_arguments_llvm::<D, 4>(arguments, context)?;
                anyhow::bail!("The `EXTCODECOPY` instruction is not supported")
            }
            Self::SELFDESTRUCT(arguments) => {
                let _arguments = Self::translate_arguments_llvm::<D, 1>(arguments, context)?;
                anyhow::bail!("The `SELFDESTRUCT` instruction is not supported")
            }

            Self::Unknown(value) => {
                anyhow::bail!("Unknown LLL instruction: {}", value);
            }
        }
    }
}
