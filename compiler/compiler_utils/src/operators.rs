//! Operator utilities

/// The different math operator types
#[derive(Debug, PartialEq, Clone)]
pub enum MathOperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
    ShiftLeft,
    ShiftRight,
    Modulo,
}

/// Represents an actual math operator
#[derive(Debug, PartialEq, Clone)]
pub struct MathOperator {
    pub operator: MathOperatorType,
    pub assigns: bool,
    pub fast: bool,
}

/// The different comparing operators
#[derive(Debug, PartialEq, Clone)]
pub enum ComparingOperator {
    Equal,       // A == B
    NotEqual,    // A != B
    Higher,      // A > B
    HigherEqual, // A >= B
    Lower,       // A < B
    LowerEqual,  // A <= B
}
