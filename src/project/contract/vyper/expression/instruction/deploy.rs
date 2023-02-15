//!
//! The `deploy` instruction.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::vyper::expression::instruction::seq::Seq as SeqInstruction;
use crate::project::contract::vyper::expression::instruction::Instruction;
use crate::project::contract::vyper::expression::Expression;

///
/// The `deploy` instruction.
///
/// It is the upper level instruction which describes the deploy code. The runtime code is located
/// in its return statement.
///
/// Since the deploy and runtime code in zkSync are not separated, they are flattened and
/// translated as entities of the same level with branching in the contract entry.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deploy([Box<Expression>; 3]);

impl Deploy {
    ///
    /// Extracts the runtime code expression from the deploy code.
    ///
    pub fn extract_runtime_code(self) -> anyhow::Result<(SeqInstruction, Expression)> {
        let [_zero_1, expression, immutables_size] = self.0;

        match *expression {
            Expression::Instruction(Instruction::Seq(sequence)) => Ok((sequence, *immutables_size)),
            expression => anyhow::bail!("Expected `seq`, found `{:?}`", expression),
        }
    }
}
