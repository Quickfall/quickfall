//!
//! AST tree related definitions.
//! 

use utils::hash::{TypeHash, WithHash};

#[derive(Debug)]
pub struct FunctionDeclarationArgument {
    pub name: WithHash<String>,
    pub argumentType: TypeHash
}

impl FunctionDeclarationArgument {
    pub fn new(name: String, argType: TypeHash) -> Self {
        FunctionDeclarationArgument { name: WithHash::new(name), argumentType: argType }
    }
}

#[derive(Debug)]
pub enum ASTTreeNode {
    IntegerLit(i64),
    StringLit(String),

    VarDeclaration { varName: WithHash<String>, varType: TypeHash, value: Option<Box<ASTTreeNode>> },
    VarValueChange { varName: WithHash<String>, value: Box<ASTTreeNode> },

    Return { value: Option<Box<ASTTreeNode>> },

    FunctionCall { funcName: WithHash<String>, args: Vec<Box<ASTTreeNode>>  },
    FunctionDeclaration { funcName: WithHash<String>, args: Vec<FunctionDeclarationArgument>, body: Vec<Box<ASTTreeNode>> }
	
}