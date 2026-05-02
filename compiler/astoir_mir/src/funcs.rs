use std::fmt::Display;

use compiler_utils::hash::HashedString;
use diagnostics::unsure_panic;
use typing::container::Type;

use crate::{
    blocks::{MIRBlockVariableSSAHint, MIRBlockVariableType, refer::MIRBlockReference},
    ctx::MIRContext,
    vals::base::BaseMIRValue,
};

/// Represents a function in the MIR. Owns one or more blocks
pub struct MIRFunction {
    /// The block storage. index 0 is entry block
    pub blocks: Vec<MIRBlockReference>,
    pub name: HashedString,

    pub id: usize,

    /// This will prevent the function from being usable by normal function calls if true
    pub is_from_struct: bool,

    pub arguments: Vec<Type>,
    pub return_type: Option<Type>,
}

impl MIRFunction {
    pub fn new(
        name: String,
        arguments: Vec<Type>,
        return_type: Option<Type>,
        is_from_struct: bool,
        id: usize,
    ) -> Self {
        return MIRFunction {
            blocks: vec![],
            name: HashedString::new(name),
            arguments,
            return_type,
            is_from_struct,
            id,
        };
    }

    pub fn append_entry_block(&mut self, ctx: &mut MIRContext) -> MIRBlockReference {
        if !self.blocks.is_empty() {
            unsure_panic!("tried using append_entry_block on a non-empty function block!");
        }

        let reference = ctx.create_block_handled(self.id);

        let block = &mut ctx.blocks[reference];

        let mut ind = 0;
        for arg in &self.arguments {
            block.variables.insert(
                ind,
                MIRBlockVariableSSAHint {
                    kind: MIRBlockVariableType::SSA,
                    hint: Some(BaseMIRValue::new(ind, arg.clone())),
                },
            );

            ind += 1;
        }

        self.blocks.push(reference);

        return reference;
    }

    pub fn append_block(&mut self, ctx: &mut MIRContext) -> MIRBlockReference {
        if self.blocks.is_empty() {
            unsure_panic!("tried using append_block on empty function blocks!");
        }

        let reference = ctx.create_block(self.id);

        return reference;
    }
}

impl Display for MIRFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".func_{}_struct{}", self.name.val, self.is_from_struct)?;

        for block in &self.blocks {
            writeln!(f, "- block_{}", block)?;
        }

        Ok(())
    }
}
