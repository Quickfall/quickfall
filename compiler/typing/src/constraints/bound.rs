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

use crate::container::Type;

pub enum BoundConstraintMember {
    Method(u64, Type, Vec<Type>),
    Field(u64, Type),
}

pub struct BoundConstraint {
    pub members: Vec<(BoundConstraintMember, bool)>,
}

impl BoundConstraint {
    pub fn new(&self) -> Self {
        BoundConstraint { members: vec![] }
    }

    /// Appends a new restriction to the bound constraint member.
    pub fn append(&mut self, member: BoundConstraintMember) {
        self.members.push((member, false));
    }

    /// Appends a new exclude-type of restriction to the bound constraint member
    pub fn append_exclude(&mut self, member: BoundConstraintMember) {
        self.members.push((member, true))
    }

    /// Checks whenever the type fits the bound constraint
    pub fn fits(&self, t: Type) -> bool {
        for member in &self.members {
            let b = match &member.0 {
                BoundConstraintMember::Field(a, b) => t.has_field(a.clone(), b.clone()),
                BoundConstraintMember::Method(a, b, c) => {
                    t.has_method(a.clone(), b.clone(), c.clone())
                }
            };

            if b == member.1 {
                return false;
            }
        }

        return true;
    }
}
