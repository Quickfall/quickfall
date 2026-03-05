//! AstoIR HIR structures related to HIR nodes

use std::collections::HashMap;

use astoir_typing::complete::ComplexType;
use compiler_errors::{IR_FIELD, errs::{BaseResult, base::BaseError}};
use compiler_utils::hash::SelfHash;

use crate::nodes::HIRNode;

pub enum StructLRUStep {
	FunctionCall { func: usize, args: Vec<Box<HIRNode>> },
	VariableStep { variable: usize }
}