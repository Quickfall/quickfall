//! Everything related to boolean handling and generation

use commons::err::{PositionlessError, PositionlessResult};

use crate::{ctx::IRContext, refs::IRValueRef, types::BOOL_TYPE_HASH, values::IRValue};

pub fn make_bool_xor<'a>(ctx: &'a IRContext<'a>, b: IRValueRef<'a>) -> PositionlessResult<IRValue<'a>> {
	let val = b.obtain(ctx)?;
	let inkwell = match val.obtain_as_bool() {
		Some(v) => v,
		None => return Err(PositionlessError::new("Cannot obtain boolean from the provided boolean value reference!"))
	};

	let bool_t = match ctx.type_storage.get(BOOL_TYPE_HASH) {
		Some(v) => v,
		None => return Err(PositionlessError::new("Cannot find boolean type in type storage!"))
	};

	let one = bool_t.get_inkwell_inttype()?.const_int(1, false);
	
	let xor_v = match ctx.builder.build_xor(inkwell, one, "xor_") {
		Ok(v) => v,
		Err(_) => return Err(PositionlessError::new("build_xor failed!"))
	};

	return Ok(IRValue::new(xor_v.into(), bool_t));
}