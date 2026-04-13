//! Operator utilities

/// The different math operators
#[derive(Debug, PartialEq, Clone)]
pub enum MathOperator {
	ADD,
	SUBSTRACT,
	MULTIPLY,
	DIVIDE
}

/// The different comparing operators
#[derive(Debug, PartialEq, Clone)]
pub enum ComparingOperator {
	Equal, // A == B
	NotEqual, // A != B
	Higher, // A > B
	HigherEqual, // A >= B
	Lower, // A < B
	LowerEqual // A <= B
}
