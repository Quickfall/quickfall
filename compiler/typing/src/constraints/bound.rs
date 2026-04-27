//! Bound constraints are used to restrict a type on if it has given fields or given methods
//! They can also be used to restrict to types that do NOT have a given field or method.
//!
//! # Example
//! ```
//! bound MyBound {
//! 	!func test(); // Matches every type that doesn't have a test function
//! 	s32 test2; // Matches every type that also has a test2 field of type s32
//! }
//! ```

use compiler_utils::hash::HashedString;

use crate::{constraints::TypeConstraint, container::Type};

#[derive(Clone)]
pub enum BoundConstraintMember {
    Method(HashedString, Type, Vec<Type>),
    Field(HashedString, Type),
}

#[derive(Clone)]
pub struct BoundConstraint {
    pub name: String,
    pub members: Vec<(BoundConstraintMember, bool)>,
}

impl BoundConstraint {
    pub fn new(name: String) -> Self {
        BoundConstraint {
            name,
            members: vec![],
        }
    }

    /// Appends a new restriction to the bound constraint member.
    pub fn append(&mut self, member: BoundConstraintMember) {
        self.members.push((member, false));
    }

    /// Appends a new exclude-type of restriction to the bound constraint member
    pub fn append_exclude(&mut self, member: BoundConstraintMember) {
        self.members.push((member, true))
    }
}

impl PartialEq for BoundConstraintMember {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Field(a, b), Self::Field(c, d)) => a == c && b == d,
            (Self::Method(a, b, c), Self::Method(d, e, f)) => a == d && b == e && c == f,
            _ => false,
        }
    }
}

impl PartialEq for BoundConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.members == other.members
    }
}

impl TypeConstraint for BoundConstraint {
    fn fits(&self, t: Type) -> bool {
        for member in &self.members {
            let b = match &member.0 {
                BoundConstraintMember::Field(a, b) => t.has_field(a.val.clone(), b.clone()),
                BoundConstraintMember::Method(a, b, c) => {
                    t.has_method(a.val.clone(), b.clone(), c.clone())
                }
            };

            if b == member.1 {
                return false;
            }
        }

        return true;
    }
}
