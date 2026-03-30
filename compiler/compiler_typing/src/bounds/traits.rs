//! # Trait bounds
//! Trait bounds are used to select types with given features. These traits can be represented with a `!` prefix. 
//! 
//! Here's a list of every trait bound with their corresponding feature:
//! - `!numeric`: Is a numeric type
//! - `!signed`: Is a signed numeric type
//! - `!integer`: Is an integer type
//! - `!floating`: Is an floating type
//! - `!fixed`: Is a fixed point type
//! - `!noninteger`: Is a non integer type like floats or fixed point numbers
//! - `!cpusupported`: Is the type supported by the CPU.
//! - `!stringlike`: Is the type a string
//! - `!static`: Is the type supposed to be statically stored
//! 
//! # Examples
//! ```
//! struct test<A: !numeric> {
//! 	..
//! }
//! ```

use compiler_utils::hash;

pub const TRAIT_NUMERIC: u64 = hash!("numeric");
pub const TRAIT_SIGNED: u64 = hash!("signed");
pub const TRAIT_INTEGER: u64 = hash!("integer");
pub const TRAIT_FLOATING: u64 = hash!("floating");
pub const TRAIT_FIXED: u64 = hash!("fixed");
pub const TRAIT_NON_INTEGER: u64 = hash!("noninteger");
pub const TRAIT_CPU_SUPPORTED: u64 = hash!("cpusupported");
pub const TRAIT_STRING: u64 = hash!("stringlike");
pub const TRAIT_STATIC: u64 = hash!("static");