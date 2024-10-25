//!
//! The `seq` instruction.
//!

use std::collections::BTreeMap;

use era_compiler_llvm_context::IContext;

use crate::project::contract::vyper::expression::instruction::label::Label as LabelInstruction;
use crate::project::contract::vyper::expression::instruction::r#return::Return as ReturnInstruction;
use crate::project::contract::vyper::expression::instruction::revert::Revert as RevertInstruction;
use crate::project::contract::vyper::expression::instruction::Instruction;
use crate::project::contract::vyper::expression::Expression;

///
/// The `seq` instruction.
///
/// This instruction contains a lot of methods to adjust the Vyper LLL structure to that of our
/// smart contract architecture or the structure of LLVM IR.
/// Among the methods there are tools of extracting the runtime code from the deploy code's
/// return statement and some logic of hoisting the contract methods to the upper levels.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct Seq(pub Vec<Expression>);

impl Seq {
    ///
    /// Checks whether the sequence only contains a single pass.
    ///
    pub fn is_pass_or_empty(&self) -> bool {
        if self.0.len() > 1 {
            return false;
        }

        match self.0.first() {
            Some(Expression::Instruction(Instruction::Pass)) => true,
            Some(_) => false,
            None => true,
        }
    }

    ///
    /// Extracts the runtime code expression from the deploy code.
    ///
    pub fn extract_runtime_code(&mut self) -> anyhow::Result<Option<(Self, Expression)>> {
        for expression in self.0.iter_mut() {
            if let Ok(Some(result)) = expression.extract_runtime_code() {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    ///
    /// Extracts the functions from the deploy or runtime code.
    ///
    pub fn extract_functions(&mut self) -> anyhow::Result<BTreeMap<String, Expression>> {
        let mut index = 0;
        let mut functions = BTreeMap::new();

        while let Some(expression) = self.0.get_mut(index) {
            if expression.is_function()? {
                let name = expression.function_name()?;
                functions.insert(name, self.0.remove(index));
            } else {
                functions.extend(expression.extract_functions()?);
                index += 1;
            }
        }

        Ok(functions)
    }

    ///
    /// Drains the expression list and splits it into labels and the rest.
    ///
    pub fn drain_and_split(&mut self) -> (Vec<LabelInstruction>, Vec<Expression>) {
        let mut labels = Vec::with_capacity(2);
        let mut expressions = Vec::with_capacity(self.0.len());

        for expression in self.0.drain(..) {
            match expression {
                Expression::Instruction(Instruction::Label(label)) => labels.push(label),
                expression => expressions.push(expression),
            }
        }

        (labels, expressions)
    }

    ///
    /// Normalizes the deploy code by inserting an empty return.
    ///
    /// If the deploy code does not have a terminator, a normal return is inserted.
    ///
    pub fn normalize_deploy_code(&mut self) {
        if self.0.is_empty()
            || matches!(
                self.0.first(),
                Some(Expression::Instruction(Instruction::Deploy(_)))
            )
        {
            self.0.push(Expression::Instruction(Instruction::RETURN(
                ReturnInstruction::default(),
            )))
        }
    }

    ///
    /// Normalizes the runtime code by inserting an empty return.
    ///
    /// If the runtime code does not have a terminator, a revert is inserted.
    ///
    pub fn normalize_runtime_code(&mut self) {
        if !self.0.is_empty() {
            return;
        }

        self.0.push(Expression::Instruction(Instruction::REVERT(
            RevertInstruction::default(),
        )))
    }

    ///
    /// Whether the sequence is a function entry block.
    ///
    pub fn is_function(&self) -> anyhow::Result<bool> {
        match self.0.first() {
            Some(Expression::Instruction(Instruction::Label(label))) => label.is_function_entry(),
            _ => Ok(false),
        }
    }

    ///
    /// Returns the function name.
    ///
    pub fn function_name(&self) -> anyhow::Result<String> {
        match self.0.first() {
            Some(Expression::Instruction(Instruction::Label(label))) => label.name(),
            expression => anyhow::bail!("Expected a function sequence, found `{expression:?}`"),
        }
    }

    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<'ctx, D>(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
    where
        D: era_compiler_llvm_context::Dependency,
    {
        let (mut labels, expressions) = self.drain_and_split();

        for label in labels.iter_mut() {
            label.declare(context)?;
        }
        for label in labels.into_iter() {
            label.into_llvm_value(context)?;
        }

        let mut result = None;
        for expression in expressions.into_iter() {
            if context.basic_block().get_terminator().is_some() {
                break;
            }

            result = expression.into_llvm_value(context)?;
        }

        Ok(result)
    }
}

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for Seq
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let current_block = context.basic_block();

        let (mut labels, expressions) = self.drain_and_split();

        for label in labels.iter_mut() {
            label.declare(context)?;
        }
        for label in labels.into_iter() {
            label.into_llvm_value(context)?;
        }

        context.set_basic_block(current_block);
        for expression in expressions.into_iter() {
            expression.into_llvm_value(context)?;
        }

        Ok(())
    }
}
