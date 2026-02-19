//! Type storage

use std::{cell::Ref, collections::HashMap, mem::transmute, ops::Add, rc::Rc};

use commons::utils::map::HashedMap;
use inkwell::{AddressSpace, context::Context, types::{IntType, PointerType}};

use crate::{ctx::IRContext, types::{BOOL_TYPE_HASH, POINTER_TYPE_HASH, SIGNED8_TYPE_HASH, SIGNED16_TYPE_HASH, SIGNED32_TYPE_HASH, SIGNED64_TYPE_HASH, SIGNED128_TYPE_HASH, UNSIGNED8_TYPE_HASH, UNSIGNED16_TYPE_HASH, UNSIGNED32_TYPE_HASH, UNSIGNED64_TYPE_HASH, UNSIGNED128_TYPE_HASH, typing::{IRType, OwnedIntType, OwnedPointerType}}, utils::SelfHash};

pub struct IRTypeStorage {
	pub map: HashMap<SelfHash, Rc<IRType>>
}

impl IRTypeStorage {
	pub fn new(ctx: &IRContext) -> Self {
		let mut sto = IRTypeStorage {map: HashMap::new()}; // 12 primitive types

		let int8 = unsafe { transmute::<IntType, IntType<'static>>(ctx.inkwell_ctx.i8_type())};
		let int16 = unsafe { transmute::<IntType, IntType<'static>>(ctx.inkwell_ctx.i16_type())};
		let int32 = unsafe { transmute::<IntType, IntType<'static>>(ctx.inkwell_ctx.i32_type())};
		let int64 = unsafe { transmute::<IntType, IntType<'static>>(ctx.inkwell_ctx.i64_type())};
		let int128 = unsafe { transmute::<IntType, IntType<'static>>(ctx.inkwell_ctx.i128_type())};

		let ptr = unsafe { transmute::<PointerType, PointerType<'static>>(ctx.inkwell_ctx.ptr_type(AddressSpace::from(0))) };
		let bool = unsafe { transmute::<IntType, IntType<'static>>(ctx.inkwell_ctx.bool_type()) };

		sto.insert(UNSIGNED8_TYPE_HASH, IRType::Unsigned8(OwnedIntType::new_ref(&ctx.inkwell_ctx, &int8)));
		sto.insert(UNSIGNED16_TYPE_HASH, IRType::Unsigned16(OwnedIntType::new(&ctx.inkwell_ctx, int16)));
		sto.insert(UNSIGNED32_TYPE_HASH, IRType::Unsigned32(OwnedIntType::new(&ctx.inkwell_ctx, int32)));
		sto.insert(UNSIGNED64_TYPE_HASH, IRType::Unsigned64(OwnedIntType::new(&ctx.inkwell_ctx, int64)));
		sto.insert(UNSIGNED128_TYPE_HASH, IRType::Unsigned128(OwnedIntType::new(&ctx.inkwell_ctx, int128)));

		sto.insert(SIGNED8_TYPE_HASH, IRType::Signed8(OwnedIntType::new_ref(&ctx.inkwell_ctx, &int8)));
		sto.insert(SIGNED16_TYPE_HASH, IRType::Signed16(OwnedIntType::new(&ctx.inkwell_ctx, int16)));
		sto.insert(SIGNED32_TYPE_HASH, IRType::Signed32(OwnedIntType::new(&ctx.inkwell_ctx, int32)));
		sto.insert(SIGNED64_TYPE_HASH, IRType::Signed64(OwnedIntType::new(&ctx.inkwell_ctx, int64)));
		sto.insert(SIGNED128_TYPE_HASH, IRType::Signed128(OwnedIntType::new(&ctx.inkwell_ctx, int128)));

		sto.insert(POINTER_TYPE_HASH, IRType::Pointer(OwnedPointerType::new(&ctx.inkwell_ctx, ptr)));

		sto.insert(BOOL_TYPE_HASH, IRType::Bool(OwnedIntType::new(&ctx.inkwell_ctx, bool)));

		return sto;
	}

	pub fn insert(&mut self, hash: u64, t: IRType) {
		self.map.insert(SelfHash { hash }, Rc::new(t));
	}

	pub fn get(&self, hash: u64) -> Option<Rc<IRType>> {
		return self.map.get(&SelfHash { hash }).cloned();
	}
}