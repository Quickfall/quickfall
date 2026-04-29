use crate::nodes::HIRNode;

#[derive(Clone)]
pub enum HIRIfBranch {
    IfBranch {
        cond: Box<HIRNode>,
        body: Vec<Box<HIRNode>>,
    },
    ElseIfBranch {
        cond: Box<HIRNode>,
        body: Vec<Box<HIRNode>>,
    },
    ElseBranch {
        body: Vec<Box<HIRNode>>,
    },
}
