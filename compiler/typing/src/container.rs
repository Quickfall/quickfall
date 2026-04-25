//! Used to represent type, uses a inner-node method to store each modifier from outside to inside.
//! Outside being the most outside modifier and inside being the raw type
//!
//!
//! # Example
//! For example the type `s32*[][32]` would be stored as:
//! ```
//! - Array(size: 32)
//! -	- Pointer(array pointer: true)
//! -	-	- Raw (signed 32 bit integer)
//! ```

/// The main container for types
pub enum Type {
    /// Represents an array of a given size and of type of the inner type container within
    Array { size: usize, inner: Box<Type> },

    /// Represents a pointer of the given inner type that is potentially an pointer-based array
    Pointer { is_array: bool, inner: Box<Type> },

    /// Represents a real raw type. A raw type is a concrete type that can be simply lowered.
    Raw {},

    /// Represents a generic type argument.
    /// A special kind of argument that passes a type parameter type as a type.
    /// It must follow the constraints given by the type parameter
    GenericTypeParam {},
}
