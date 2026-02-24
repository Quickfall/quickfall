//! IR representation of structure types (structs, layouts...)

use std::{mem::transmute, rc::Rc};

use commons::{utils::map::HashedMap};
use errors::{INKWELL_FUNC_FAILED, IR_FIELD, IR_FIND_FUNCTION, IR_LAYOUT_FUNC_USAGE, IR_LAYOUT_FUNCS, errs::{BaseResult, base::BaseError}};
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
	pub fn new(ctx: &IRContext, name: String, layout: bool, fields: Vec<(u64, Rc<IRType>)>) -> BaseResult<Self>  {
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

	pub fn append_function(&mut self, hash: u64, func: IRFunction) -> BaseResult<bool> {
		if self.is_layout {
			return Err(BaseError::err(IR_LAYOUT_FUNCS!().to_string()));
		}

		self.functions.put(hash, Rc::new(func));
		return Ok(true);
	}

	pub fn get_function(&self, hash: u64) -> BaseResult<Rc<IRFunction>> {
		if self.is_layout {
			return Err(BaseError::err(IR_LAYOUT_FUNC_USAGE!().to_string()));
		}

		return match self.functions.get(hash) {
			Some(v) => Ok(v.clone()),
			None => Err(BaseError::err(IR_FIND_FUNCTION!().to_string()))
		}
	}

	pub fn get_pointer_for_field_index(&self, ctx: &IRContext, instance: &IRPointer, ind: u32) -> BaseResult<IRPointer> {
		if ind >= self.field_types.len() as u32 {
			return Err(BaseError::err("Invalid index given to get_pointer_for_field_index".to_string()));
		}

		let field_ptr = match ctx.builder.build_struct_gep(self.inkwell_type, instance.inkwell_ptr, ind, "field_ptr") {
			Ok(v) => v,
			Err(e) => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "build_struct_gep", e)))
		};

		let field_type = self.field_types[ind as usize].clone();

		return Ok(IRPointer::new(field_ptr, ctx, field_type, String::from("__inner_field_ptr")));
	}

	pub fn get_pointer_for_field_index_noref(&self, ctx: &IRContext, instance: IRPointer, ind: u32) -> BaseResult<IRPointer> {
		if ind >= self.field_types.len() as u32 {
			return Err(BaseError::err("Invalid index given to get_pointer_for_field_index".to_string()));
		}

		let field_ptr = match ctx.builder.build_struct_gep(self.inkwell_type, instance.inkwell_ptr, ind, "field_ptr") {
			Ok(v) => v,
			Err(e) => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "build_struct_gep", e)))
		};

		let field_type = self.field_types[ind as usize].clone();

		return Ok(IRPointer::new(field_ptr, ctx, field_type, String::from("__inner_field_ptr")));
	}

	pub fn get_pointer_for_field_noref(&self, ctx: &IRContext, instance: IRPointer, hash: u64) -> BaseResult<IRPointer> {
		let k = match self.field_to_index.get(hash) {
			Some(v) => *v,
			None => return Err(BaseError::err(IR_FIELD!().to_string()))
		};

		return self.get_pointer_for_field_index_noref(ctx, instance, k);
	}

	pub fn get_pointer_for_field(&self, ctx: &IRContext, instance: &IRPointer, hash: u64) -> BaseResult<IRPointer> {
		let k = match self.field_to_index.get(hash) {
			Some(v) => *v,
			None => return Err(BaseError::err(IR_FIELD!().to_string()))
		};

		return self.get_pointer_for_field_index(ctx, instance, k);
	}
} 
