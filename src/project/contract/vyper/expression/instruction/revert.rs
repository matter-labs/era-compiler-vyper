//!
//! The `revert` instruction.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::vyper::expression::instruction::Instruction;
use crate::project::contract::vyper::expression::Expression;

///
/// The `revert` instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn into_llvm_value<D>(
        self,
        context: &mut compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()>
    where
        D: compiler_llvm_context::EraVMDependency + Clone,
    {
        let arguments = Instruction::translate_arguments_llvm::<D, 2>(self.0, context)?;
        compiler_llvm_context::eravm_evm_return::revert(
            context,
            arguments[0].into_int_value(),
            arguments[1].into_int_value(),
        )
    }
}
