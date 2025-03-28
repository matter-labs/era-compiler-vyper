//!
//! The `revert` instruction.
//!

use crate::project::contract::vyper::expression::instruction::Instruction;
use crate::project::contract::vyper::expression::Expression;

///
/// The `revert` instruction.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Revert([Box<Expression>; 2]);

impl Default for Revert {
    fn default() -> Self {
        let offset = Expression::IntegerLiteral(serde_json::Number::from(0));
        let size = Expression::IntegerLiteral(serde_json::Number::from(0));
        Self([Box::new(offset), Box::new(size)])
    }
}

impl Revert {
    ///
    /// Converts the entity to an LLVM value.
    ///
    pub fn into_llvm_value(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let arguments = Instruction::translate_arguments_llvm::<2>(self.0, context)?;
        era_compiler_llvm_context::eravm_evm_return::revert(
            context,
            arguments[0].into_int_value(),
            arguments[1].into_int_value(),
        )
    }
}
