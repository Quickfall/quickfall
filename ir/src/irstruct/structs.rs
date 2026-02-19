//! IR representation of structure types (structs, layouts...)

use std::{mem::transmute, rc::Rc};

use commons::{err::{PositionlessError, PositionlessResult}, utils::map::HashedMap};
use inkwell::{context::Context, types::{BasicTypeEnum, StructType}};

use crate::{ctx::IRContext, irstruct::{funcs::IRFunction, ptr::IRPointer}, types::typing::{IRType, OwnedTypeEnum}};

pub struct IRStructuredType {
	pub owned: Rc<Context>,
	pub inkwell_type: StructType<'static>,
	pub field_to_index: HashedMap<u32>,
	pub field_types: Vec<Rc<IRType>>,
	pub functions: HashedMap<Rc<IRFunction>>,
	pub name: String,
	pub is_layout: bool
}

impl IRStructuredType {
	pub fn new(ctx: &IRContext, name: String, layout: bool, fields: Vec<(u64, Rc<IRType>)>) -> PositionlessResult<Self>  {
		let mut map = HashedMap::new(fields.len());
		let mut typeVec: Vec<BasicTypeEnum> = Vec::new();
		let mut field_types: Vec<Rc<IRType>> = Vec::new();

		let mut ind = 0;
		for entry in fields {
			map.put(entry.0, ind);

			field_types.push(entry.1.clone());
			typeVec.push(entry.1.as_ref().get_inkwell_basetype()?.inner.into());

			ind += ind;
		}

		let inkwell_type = ctx.inkwell_ctx.struct_type(&typeVec, !layout);

		return Ok(Self { owned: ctx.inkwell_ctx.clone(), inkwell_type: unsafe { transmute(inkwell_type) }, field_to_index: map, name, is_layout: layout, field_types, functions: HashedMap::new(0) })
	}

	pub fn append_function(&mut self, hash: u64, func: IRFunction) -> PositionlessResult<bool> {
		if self.is_layout {
			return Err(PositionlessError::new("Cannot declare functions inside of a layout!"));
		}

		self.functions.put(hash, Rc::new(func));
		return Ok(true);
	}

	pub fn get_function(&self, hash: u64) -> PositionlessResult<Rc<IRFunction>> {
		if self.is_layout {
			return Err(PositionlessError::new("Cannot use typed-functions inside of a layout!"));
		}

		return match self.functions.get(hash) {
			Some(v) => Ok(v.clone()),
			None => Err(PositionlessError::new("Function was not founc within the struct!"))
		}
	}

	pub fn get_pointer_for_field_index(&self, ctx: &IRContext, instance: &IRPointer, ind: u32) -> PositionlessResult<IRPointer> {
		if ind >= self.field_types.len() as u32 {
			return Err(PositionlessError::new("Invalid index given to get_pointer_for_field_index"));
		}

		let field_ptr = match ctx.builder.build_struct_gep(self.inkwell_type, instance.inkwell_ptr, ind, "field_ptr") {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_struct_gep failed!"))
		};

		let field_type = self.field_types[ind as usize].clone();

		return Ok(IRPointer::new(field_ptr, ctx, field_type, String::from("__inner_field_ptr")));
	}

	pub fn get_pointer_for_field_index_noref(&self, ctx: &IRContext, instance: IRPointer, ind: u32) -> PositionlessResult<IRPointer> {
		if ind >= self.field_types.len() as u32 {
			return Err(PositionlessError::new("Invalid index given to get_pointer_for_field_index"));
		}

		let field_ptr = match ctx.builder.build_struct_gep(self.inkwell_type, instance.inkwell_ptr, ind, "field_ptr") {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_struct_gep failed!"))
		};

		let field_type = self.field_types[ind as usize].clone();

		return Ok(IRPointer::new(field_ptr, ctx, field_type, String::from("__inner_field_ptr")));
	}

	pub fn get_pointer_for_field_noref(&self, ctx: &IRContext, instance: IRPointer, hash: u64) -> PositionlessResult<IRPointer> {
		let k = match self.field_to_index.get(hash) {
			Some(v) => *v,
			None => return Err(PositionlessError::new(&format!("The given string hash {} doesn't represent any field in the struct {}", hash, self.name)))
		};

		return self.get_pointer_for_field_index_noref(ctx, instance, k);
	}

	pub fn get_pointer_for_field(&self, ctx: &IRContext, instance: &IRPointer, hash: u64) -> PositionlessResult<IRPointer> {
		let k = match self.field_to_index.get(hash) {
			Some(v) => *v,
			None => return Err(PositionlessError::new(&format!("The given string hash {} doesn't represent any field in the struct {}", hash, self.name)))
		};

		return self.get_pointer_for_field_index(ctx, instance, k);
	}
} 
