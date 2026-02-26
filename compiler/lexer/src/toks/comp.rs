//! Comparing token related utils

/// The different comparing tokens
#[derive(Debug, PartialEq, Clone)]
pub enum ComparingOperator {
	Equal, // A == B
	NotEqual, // A != B
	Higher, // A > B
	HigherEqual, // A >= B
	Lower, // A < B
	LowerEqual // A <= B
}