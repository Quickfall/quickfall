//! Comparing token related utils

/// The different comparing tokens
pub enum ComparingOperator {
	EQUAL, // A == B
	N_EQUAL, // A != B
	HIGHER, // A > B
	HIGHER_EQ, // A >= B
	LOWER, // A < B
	LOWER_EQ // A <= B
}