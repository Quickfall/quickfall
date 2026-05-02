use std::fmt::Display;

use diagnostics::{DiagnosticResult, builders::make_invalid_var_type_ir};

use crate::vals::base::BaseMIRValue;

#[derive(Clone)]
pub struct MIRPointerValue {
    base: BaseMIRValue,
}

impl MIRPointerValue {
    pub fn new(base: BaseMIRValue) -> DiagnosticResult<Self> {
        if base.can_be_pointer() {
            return Ok(MIRPointerValue { base: base.clone() });
        }

        return Err(make_invalid_var_type_ir().into());
    }
}

impl Into<BaseMIRValue> for MIRPointerValue {
    fn into(self) -> BaseMIRValue {
        return self.base;
    }
}

impl Display for MIRPointerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.base.get_ssa_index())?;

        Ok(())
    }
}
