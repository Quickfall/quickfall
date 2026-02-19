//! Static variable related code

use std::rc::Rc;

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::{builder::Builder, values::{BasicValueEnum, GlobalValue, IntValue}};

use crate::{ctx::IRContext, types::{SIGNED8_TYPE_HASH, typing::{IRType, OwnedGlobalValue, OwnedValueEnum}}, values::IRValue};

#[derive(Clone)]
pub struct IRStaticVariable {
	inkwell: Option<OwnedGlobalValue>,
	val: Option<OwnedValueEnum>,
	pub t: Rc<IRType>,
	pub name: String
}

impl IRStaticVariable {
	pub fn from_str(ctx: &IRContext, str: &str, name: String, t: Rc<IRType>) -> PositionlessResult<IRStaticVariable> {
		let bytes = str.as_bytes();

		let byte_type = ctx.type_storage.get(SIGNED8_TYPE_HASH).expect("Cannot find i8 in type storage!");
		let i8_type = byte_type.get_inkwell_inttype()?;

		let array_type = i8_type.array_type((bytes.len() + 1) as u32);

		let global = ctx.module.add_global(array_type, None, &name);

		global.set_linkage(inkwell::module::Linkage::Private);
		global.set_constant(true);
		global.set_unnamed_addr(true);

		let mut vals: Vec<IntValue> = bytes.iter().map(|b| i8_type.const_int(*b as u64, false)).collect();

		vals.push(i8_type.const_zero());

		global.set_initializer(&i8_type.const_array(&vals));

		return Ok(IRStaticVariable { inkwell: Some(OwnedGlobalValue::new(&ctx.inkwell_ctx, global)), t, name, val: None });
	}

	pub fn from_val(name: String, t: Rc<IRType>, val: IRValue) -> PositionlessResult<IRStaticVariable> {
		return Ok(IRStaticVariable { val: Some(val.obtain()), inkwell: None, t, name })
	}

	pub fn is_compiletime_replaceable(&self) -> bool {
		return self.val.is_some();
	}

	pub fn as_val(&self) -> PositionlessResult<OwnedValueEnum> {
		if self.val.is_some() {
			return Ok(self.val.as_ref().unwrap().clone());
		}

		return Ok(OwnedValueEnum::new(&self.inkwell.as_ref().unwrap().owned, self.as_string_ref()?.as_pointer_value().into()));
	}

	pub fn as_string_ref(&self) -> PositionlessResult<OwnedGlobalValue> {
		if self.is_compiletime_replaceable() {
			return Err(PositionlessError::new("Tried using as_string_ref on a compiletime determined global var"));
		}

		return Ok(self.inkwell.clone().unwrap())
	}

}