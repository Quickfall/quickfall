use crate::tree::ASTTreeNode;

#[derive(Debug, PartialEq, Clone)]
pub struct ASTRange {
    pub min: Box<ASTTreeNode>,
    pub max: Box<ASTTreeNode>,
}
