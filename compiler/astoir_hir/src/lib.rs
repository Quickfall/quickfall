//! The core of the HIR layer of AstoIR.

pub mod context;
pub mod nodes;
pub mod ifelse;
pub mod lru;
pub mod func;

pub trait PureCompTimeCandidate {
    /// Is the given node pure? Meaning can it be used inside of a pure function.
    /// A pure function requires the following:
    /// - Can be ran at compile time
    /// - No inside function calls that aren't recursive
    fn is_pure(&self) -> bool;

    /// Is the given node compile time compatible? Meaning can it be used of a compile time function.
    /// A compile time function requires the following:
    /// - Can be ran at compile time
    /// - Every variable must be knowable at compile time
    /// - No memory allocation / deallocation
    /// - No pointers
    fn is_comptime(&self) -> bool;
}
