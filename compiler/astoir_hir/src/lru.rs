use crate::nodes::HIRNode;

#[derive(Clone)]
pub enum StructLRUStep {
    FunctionCall {
        func: usize,
        args: Vec<Box<HIRNode>>,
    },
    VariableStep {
        variable: usize,
    },
}
