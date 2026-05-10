use std::fmt::Display;

use compiler_typing::raw::RawType;
use diagnostics::{DiagnosticResult, builders::make_invalid_var_type_ir};

use crate::vals::base::BaseMIRValue;

#[derive(Clone, Debug)]
pub struct MIRPointerValue {
    base: BaseMIRValue,
}

impl MIRPointerValue {
    pub fn new(base: BaseMIRValue) -> DiagnosticResult<Self> {
        if base.vtype.is_technically_pointer()
            || base.vtype.is_array()
            || base.vtype.as_generic_lowered() == RawType::Pointer
        {
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
        self.base.fmt(f)
    }
}
