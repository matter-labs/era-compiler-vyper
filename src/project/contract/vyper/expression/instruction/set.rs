//!
//! The `set` instruction.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper LLL-specific `set` instruction.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Set([Box<Expression>; 2]);

impl Set {
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
        let [identifier, value] = self.0;
        let identifier = identifier.try_into_identifier()?;

        let pointer = context
            .current_function()
            .borrow()
            .get_stack_pointer(identifier.as_str())
            .ok_or_else(|| anyhow::anyhow!("Variable `{}` not found", identifier))?;

        let value = value
            .into_llvm_value(context)?
            .ok_or_else(|| anyhow::anyhow!("Expected a value"))?;
        context.build_store(pointer, value);

        Ok(())
    }
}
