//! Feature constraints are a way to restrict a type to if it has or has not a given feature (eg: is an integer type)
//! Here's a list of feature constraints:
//! - !numeric
//! - !signed
//! - !integer
//! - !floating
//! - !fixed
//! - !noninteger
//! - !cpusupported
//! - !stringlike
//! - !static
//! - !mathoperations
//! - !struct
//!
//! Additionally the exclude variant can be used by replacing the ! by a #.
//!
//! # Examples
//! ```
//! struct test<A: !numeric !mathoperations> {
//! 	A myFirstNumber // Here number will always be a numeric type WITH mathematical operations instead of custom operators
//! }
//! ```
//!
pub enum FeatureFlag {
    /// Is the type a numeric type (holds a number directly)
    Numeric,

    /// Is the type a signed numeric type
    Signed,

    /// Is the type an integer
    Integer,

    /// Is the type a floating point number
    Floating,

    /// Is the type a fixed point number
    Fixed,

    /// Is the type a non integer number (floating + fixed)
    NonInteger,

    /// Is the type natively supported by the CPU
    CpuSupported,

    /// Is the type like a string
    StringLike,

    /// Is the type static
    Static,

    /// Does the type use native math operations instead of custom operators
    MathOperations,

    /// Is the type a structure like type
    Struct,
}
