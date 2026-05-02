use std::fmt::Display;

use diagnostics::DiagnosticResult;
use typing::container::Type;

use crate::vals::{
    arrays::MIRArrayValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue,
    structs::MIRStructValue,
};

/// Represents a basic value in the MIR.
#[derive(Clone)]
pub struct BaseMIRValue {
    val_index: usize,
    pub vtype: Type,
}

impl BaseMIRValue {
    pub fn new(val_index: usize, vtype: Type) -> Self {
        return BaseMIRValue { val_index, vtype };
    }

    pub fn as_int(&self) -> DiagnosticResult<MIRIntValue> {
        return Ok(MIRIntValue::new(self.clone())?);
    }

    pub fn as_float(&self) -> DiagnosticResult<MIRFloatValue> {
        return Ok(MIRFloatValue::new(self.clone())?);
    }

    pub fn as_ptr(&self) -> DiagnosticResult<MIRPointerValue> {
        return Ok(MIRPointerValue::new(self.clone())?);
    }

    pub fn as_struct(&self) -> DiagnosticResult<MIRStructValue> {
        return Ok(MIRStructValue::new(self.clone())?);
    }

    pub fn as_array(&self) -> DiagnosticResult<MIRArrayValue> {
        return Ok(MIRArrayValue::new(self.clone())?);
    }

    pub fn can_be_pointer(&self) -> bool {
        return self.vtype.is_ptr() || self.vtype.is_array();
    }

    pub fn get_ssa_index(&self) -> usize {
        return self.val_index;
    }
}

impl PartialEq for BaseMIRValue {
    fn eq(&self, other: &Self) -> bool {
        return self.val_index == other.val_index && self.vtype == other.vtype;
    }
}

impl Display for BaseMIRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.val_index)?;

        Ok(())
    }
}
