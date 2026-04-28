use compiler_typing::tree::Type;
use diagnostics::unsure_panic;

use crate::vals::consts::MIRConstantValue;

/// A hint on a given value, contains constants or pointer types for example
#[derive(Clone, Debug)]
pub enum MIRValueHint {
    Constant(MIRConstantValue),
    Pointer(Type),
    Value(Type),
}

impl MIRValueHint {
    pub fn is_determined(&self) -> bool {
        if let &MIRValueHint::Constant(_) = self {
            return true;
        }

        return false;
    }

    pub fn is_pointer(&self) -> bool {
        if let &MIRValueHint::Pointer(_) = self {
            return true;
        }

        return false;
    }

    pub fn as_const(&self) -> MIRConstantValue {
        match self {
            MIRValueHint::Constant(e) => e.clone(),
            _ => unsure_panic!("cannot use as_const on a non const!"),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            MIRValueHint::Pointer(e) => e.clone(),
            MIRValueHint::Value(e) => e.clone(),
            _ => unsure_panic!("cannot use get_type on a non typed hint"),
        }
    }

    pub fn as_pointer(&self) -> Type {
        match self {
            MIRValueHint::Pointer(e) => e.clone(),
            _ => unsure_panic!("Cannot use as_pointer on a non pointer!"),
        }
    }

    pub fn as_value(&self) -> Type {
        match self {
            MIRValueHint::Value(e) => e.clone(),
            _ => unsure_panic!("Cannot use as_value on a non value!"),
        }
    }

    pub fn from_ptr(val: Type) -> Self {
        return MIRValueHint::Pointer(val);
    }
}

impl Into<MIRValueHint> for MIRConstantValue {
    fn into(self) -> MIRValueHint {
        return MIRValueHint::Constant(self);
    }
}

impl Into<MIRValueHint> for Type {
    fn into(self) -> MIRValueHint {
        return MIRValueHint::Value(self);
    }
}

pub struct HintStorage {
    pub vec: Vec<MIRValueHint>,
}

impl HintStorage {
    pub fn new() -> Self {
        HintStorage { vec: vec![] }
    }

    /// Introduces a new SSA value hint. Returns the hint index.
    /// # Usage
    /// Every single SSA value should have a hint on what it is. Furthermore, this hint index will be used to identify the different SSA values instead of raw instruction indexes.
    ///
    /// # Globality
    /// Using hint indexes to represent different SSA values allows us to guarantee that SSA values will work on inner blocks.
    pub fn append_hint(&mut self, hint: MIRValueHint) -> usize {
        let ind = self.vec.len();

        self.vec.push(hint);

        return ind;
    }

    /// Gets the hint based on the hint index.
    pub fn get_hint(&self, hint_ind: usize) -> MIRValueHint {
        if self.vec.len() <= hint_ind {
            unsure_panic!("invalid hint provided");
        }

        return self.vec[hint_ind].clone();
    }
}
