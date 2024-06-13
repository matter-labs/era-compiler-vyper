//!
//! The `return` instruction.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::vyper::expression::instruction::Instruction;
use crate::project::contract::vyper::expression::Expression;

///
/// The `return` instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Return([Box<Expression>; 2]);

impl Default for Return {
    fn default() -> Self {
        let offset = Expression::IntegerLiteral(serde_json::Number::from(0));
        let size = Expression::IntegerLiteral(serde_json::Number::from(0));
        Self([Box::new(offset), Box::new(size)])
    }
}

impl Return {
    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value<D>(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()>
    where
        D: era_compiler_llvm_context::Dependency,
    {
        let arguments = Instruction::translate_arguments_llvm::<D, 2>(self.0, context)?;
        era_compiler_llvm_context::eravm_evm_return::r#return(
            context,
            arguments[0].into_int_value(),
            arguments[1].into_int_value(),
        )
    }
}
