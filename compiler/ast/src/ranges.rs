use crate::tree::ASTTreeNode;

pub struct ASTRange {
    pub min: Box<ASTTreeNode>,
    pub max: Box<ASTTreeNode>,
}
