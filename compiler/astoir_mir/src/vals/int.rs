use std::fmt::Display;

use diagnostics::{DiagnosticResult, builders::make_invalid_var_type_ir};
use typing::raw::RawType;

use crate::vals::base::BaseMIRValue;

#[derive(Clone)]
pub struct MIRIntValue {
    pub base: BaseMIRValue,
    pub signed: bool,
    pub size: usize,
}

impl MIRIntValue {
    pub fn new(base: BaseMIRValue) -> DiagnosticResult<Self> {
        if let RawType::Integer(signed, size) = base.vtype.clone().get_raw().t {
            return Ok(MIRIntValue {
                base: base.clone(),
                size,
                signed,
            });
        }

        if let RawType::Boolean = base.vtype.clone().get_raw().t {
            return Ok(MIRIntValue {
                base: base.clone(),
                signed: false,
                size: 1,
            });
        }

        return Err(make_invalid_var_type_ir().into());
    }
}

impl Into<BaseMIRValue> for MIRIntValue {
    fn into(self) -> BaseMIRValue {
        return self.base;
    }
}

impl Display for MIRIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.base.get_ssa_index())?;

        Ok(())
    }
}
