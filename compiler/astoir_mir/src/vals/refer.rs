use compiler_typing::TypedGlobalScope;
use diagnostics::{DiagnosticResult, builders::make_invalid_assign_diff_type_ir, unsure_panic};

use crate::{
    blocks::refer::MIRBlockReference,
    builder::{build_load, build_store},
    ctx::MIRContext,
    vals::{base::BaseMIRValue, ptr::MIRPointerValue},
};

/// Represents a reference to a variable. This is not a reference to individual SSA values.
///
/// # Variables
/// Variables can be either:
/// - A pointer (using `store` and `load`)
/// - SSA handled variable (stores variable ID -> current SSA value for the variable)
///
/// # Finding Guarantee
/// This reference guarantees to point to valid variable within a function. Please note that a reference can **only** be used in the function it was created in.
/// No Safety has been put in place to limit these errors as these references should never leave their origin function anyways
///
/// # Usage
/// This can be used to reference any true variable to modify it's content or read it. This will automatically handle diverse instructions needed such as `load` and `store`.
#[derive(Debug)]
pub enum MIRVariableReference {
    PointerReference(MIRPointerValue),
    SSAReference(usize),
}

impl MIRVariableReference {
    pub fn read(
        &self,
        block: MIRBlockReference,
        ctx: &mut MIRContext,
    ) -> DiagnosticResult<BaseMIRValue> {
        if self.is_pointer_ref() {
            let ptr_ref = self.as_pointer_ref()?;

            let res = build_load(ctx, ptr_ref)?;

            return Ok(res);
        }

        let ind = self.as_ssa_ref()?;

        return match &ctx.blocks[block].variables[&ind].hint {
            Some(v) => Ok(v.clone()),
            None => unsure_panic!(
                "cannot unpack SSA reference for variable in MIRVariableReference::read"
            ),
        };
    }

    pub fn write(
        &self,
        block: MIRBlockReference,
        ctx: &mut MIRContext,
        val: BaseMIRValue,
        storage: &TypedGlobalScope,
    ) -> DiagnosticResult<bool> {
        if self.is_pointer_ref() {
            let ptr_ref = self.as_pointer_ref()?;

            //let _hint = ctx
            //    .ssa_hints
            //    .get_hint(BaseMIRValue::from(ptr_ref.clone().into()).get_ssa_index());

            //if hint.get_type().is_technically_pointer() {
            //    ptr_ref = build_load(ctx, ptr_ref)?.as_ptr()?;
            //    println!("Triggered {}", val);
            //}

            build_store(ctx, storage, ptr_ref, val)?;

            return Ok(true);
        }

        let ind = self.as_ssa_ref()?;

        let block = &mut ctx.blocks[block];

        if block.variables[&ind].hint.is_some()
            && !block.variables[&ind]
                .hint
                .clone()
                .unwrap()
                .vtype
                .is_truly_eq(&val.vtype)
        {
            return Err(make_invalid_assign_diff_type_ir().into());
        }

        let mut hint = block.variables[&ind].clone();
        hint.hint = Some(val);

        block.variables.insert(ind, hint);

        return Ok(true);
    }

    pub fn get_hint(&self) -> usize {
        return match self {
            Self::PointerReference(e) => {
                let clone: BaseMIRValue = e.clone().into();

                return clone.get_ssa_index();
            }
            Self::SSAReference(e) => *e,
        };
    }

    pub fn is_pointer_ref(&self) -> bool {
        return match self {
            Self::PointerReference(_) => true,
            _ => false,
        };
    }

    pub fn as_pointer_ref(&self) -> DiagnosticResult<MIRPointerValue> {
        return match self {
            Self::PointerReference(e) => Ok(e.clone()),
            _ => unsure_panic!("as_pointer_ref requires a pointer var ref!"),
        };
    }

    pub fn as_ssa_ref(&self) -> DiagnosticResult<usize> {
        return match self {
            Self::SSAReference(e) => Ok(*e),
            _ => unsure_panic!("as_ssa_ref requires a SSA var ref!"),
        };
    }

    pub fn is_ssa_ref(&self) -> bool {
        return !self.is_pointer_ref();
    }
}

impl From<MIRPointerValue> for MIRVariableReference {
    fn from(value: MIRPointerValue) -> Self {
        return MIRVariableReference::PointerReference(value);
    }
}

impl From<usize> for MIRVariableReference {
    fn from(value: usize) -> Self {
        return MIRVariableReference::SSAReference(value);
    }
}
