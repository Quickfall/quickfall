//! Type storage

use std::rc::Rc;

use commons::utils::map::HashedMap;
use inkwell::{AddressSpace, context::Context};

use crate::types::{BOOL_TYPE_HASH, POINTER_TYPE_HASH, SIGNED8_TYPE_HASH, SIGNED16_TYPE_HASH, SIGNED32_TYPE_HASH, SIGNED64_TYPE_HASH, SIGNED128_TYPE_HASH, UNSIGNED8_TYPE_HASH, UNSIGNED16_TYPE_HASH, UNSIGNED32_TYPE_HASH, UNSIGNED64_TYPE_HASH, UNSIGNED128_TYPE_HASH, typing::IRType};

pub struct IRTypeStorage {
	map: HashedMap<Rc<IRType>>
}

impl IRTypeStorage {
	pub fn new(ctx: Context) -> Self {
		let mut sto = IRTypeStorage {map: HashedMap::new(12)}; // 12 primitive types

		sto.insert(UNSIGNED8_TYPE_HASH, IRType::Unsigned8(ctx.i8_type()));
		sto.insert(UNSIGNED16_TYPE_HASH, IRType::Unsigned16(ctx.i16_type()));
		sto.insert(UNSIGNED32_TYPE_HASH, IRType::Unsigned32(ctx.i32_type()));
		sto.insert(UNSIGNED64_TYPE_HASH, IRType::Unsigned64(ctx.i64_type()));
		sto.insert(UNSIGNED128_TYPE_HASH, IRType::Unsigned128(ctx.i128_type()));

		sto.insert(SIGNED8_TYPE_HASH, IRType::Signed8(ctx.i8_type()));
		sto.insert(SIGNED16_TYPE_HASH, IRType::Signed16(ctx.i16_type()));
		sto.insert(SIGNED32_TYPE_HASH, IRType::Signed32(ctx.i32_type()));
		sto.insert(SIGNED64_TYPE_HASH, IRType::Signed64(ctx.i64_type()));
		sto.insert(SIGNED128_TYPE_HASH, IRType::Signed128(ctx.i128_type()));

		sto.insert(POINTER_TYPE_HASH, IRType::Pointer(ctx.ptr_type(AddressSpace::from(0u16))));

		sto.insert(BOOL_TYPE_HASH, IRType::Bool(ctx.bool_type()));

		return sto;
	}

	pub fn insert(&mut self, hash: u64, t: IRType) {
		self.map.put(hash, Rc::new(t));
	}

	pub fn get(&self, hash: u64) -> Option<Rc<IRType>> {
		return self.map.get(hash).cloned();
	}
}