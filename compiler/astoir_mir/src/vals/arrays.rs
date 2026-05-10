use std::fmt::Display;

use compiler_typing::tree::Type;
use diagnostics::{DiagnosticResult, builders::make_invalid_var_type_ir};

use crate::vals::base::BaseMIRValue;

pub struct MIRArrayValue {
    pub base: BaseMIRValue,
    pub size: usize,
}

impl MIRArrayValue {
    pub fn new(base: BaseMIRValue) -> DiagnosticResult<Self> {
        if let Type::Array(size, _) = base.vtype.clone() {
            return Ok(MIRArrayValue { base, size });
        }

        return Err(make_invalid_var_type_ir().into());
    }
}

impl Into<BaseMIRValue> for MIRArrayValue {
    fn into(self) -> BaseMIRValue {
        return self.base;
    }
}

impl Display for MIRArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}
