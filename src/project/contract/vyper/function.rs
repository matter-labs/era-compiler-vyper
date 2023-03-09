//!
//! The Vyper contract function representation.
//!

use inkwell::types::BasicType;

use compiler_llvm_context::Dependency;
use compiler_llvm_context::WriteLLVM;

use crate::metadata::function::Function as FunctionMetadata;
use crate::project::contract::vyper::expression::Expression;

///
/// The Vyper contract function representation.
///
#[derive(Debug)]
pub struct Function {
    /// The function name.
    pub name: String,
    /// The function metadata.
    pub metadata: Option<FunctionMetadata>,
    /// The function body expression.
    pub expression: Expression,
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, metadata: Option<FunctionMetadata>, expression: Expression) -> Self {
        Self {
            name,
            metadata,
            expression,
        }
    }
}

impl<D> WriteLLVM<D> for Function
where
    D: Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let mut argument_types = vec![];
        if self
            .name
            .starts_with(crate::r#const::FUNCTION_PREFIX_INTERNAL)
        {
            if let Some(ref metadata) = self.metadata {
                if !metadata.return_type.is_empty() && metadata.return_type != "None" {
                    argument_types.push(context.field_type().as_basic_type_enum());
                }
            }
        }

        let function = context.add_function(
            self.name.as_str(),
            context.function_type(argument_types, 0, false),
            0,
            Some(inkwell::module::Linkage::Private),
        )?;
        function
            .borrow_mut()
            .set_vyper_data(compiler_llvm_context::FunctionVyperData::default());

        Ok(())
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        context.set_current_function(self.name.as_str())?;

        let llvm_entry_block = context.current_function().borrow().entry_block();
        let llvm_value = context.current_function().borrow().declaration().value;

        self.expression.into_llvm_value(context)?;

        context.set_basic_block(llvm_entry_block);
        let ir_entry_block = llvm_value
            .get_basic_blocks()
            .iter()
            .find(|block| block.get_name().to_string_lossy() == self.name)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Block `{}` does not exist", self.name))?;
        context.build_unconditional_branch(ir_entry_block);

        context.set_basic_block(context.current_function().borrow().return_block());
        context.build_return(None);

        for block in llvm_value.get_basic_blocks() {
            if block.get_terminator().is_none() {
                context.set_basic_block(block);
                context.build_exit(
                    context.intrinsics().revert,
                    context.field_const(0),
                    context.field_const(0),
                );
            }
        }

        Ok(())
    }
}
