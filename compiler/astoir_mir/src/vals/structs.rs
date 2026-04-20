use std::fmt::Display;

use compiler_typing::raw::RawType;
use diagnostics::{DiagnosticResult, builders::make_invalid_var_type_ir};

use crate::vals::base::BaseMIRValue;

pub struct MIRStructValue {
    pub base: BaseMIRValue,
    pub t: RawType,
}

impl MIRStructValue {
    pub fn new(base: BaseMIRValue) -> DiagnosticResult<Self> {
        if let RawType::LoweredStruct(_, _) = base.vtype.clone().as_generic_lowered() {
            return Ok(MIRStructValue {
                base: base.clone(),
                t: base.vtype.clone().as_generic_lowered(),
            });
        }

        return Err(make_invalid_var_type_ir().into());
    }
}

impl Into<BaseMIRValue> for MIRStructValue {
    fn into(self) -> BaseMIRValue {
        return self.base;
    }
}

impl Display for MIRStructValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.base.get_ssa_index())?;

        Ok(())
    }
}
