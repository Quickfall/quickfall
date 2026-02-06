///! Maths token related utils

/// The different operators
#[derive(Debug, PartialEq, Clone)]
pub enum MathOperator {
	ADD,
	SUBSTRACT,
	MULTIPLY,
	DIVIDE
}