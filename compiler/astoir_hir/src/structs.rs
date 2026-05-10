//! AstoIR HIR structures related to HIR nodes

use crate::nodes::HIRNode;

#[derive(Debug, Clone)]
pub enum StructLRUStep {
    FunctionCall {
        func: usize,
        args: Vec<Box<HIRNode>>,
    },
    VariableStep {
        variable: usize,
    },
}

#[derive(Debug, Clone)]
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

#[derive(Clone, Debug)]
pub struct HIRRange {
    pub min: Box<HIRNode>,
    pub max: Box<HIRNode>,
}

#[derive(Debug)]
pub struct HIRStructContainer {
    pub function_impls: Vec<Box<HIRNode>>,
}
