//! IR representation of structure types (structs, layouts...)

use commons::{err::{PositionlessError, PositionlessResult}, utils::map::HashedMap};
use inkwell::{types::{BasicTypeEnum, StructType}};

use crate::{ctx::IRContext, irstruct::{funcs::IRFunction, ptr::IRPointer}, types::typing::IRType};

pub struct IRStructuredType<'a> {
	pub inkwell_type: StructType<'a>,
	pub field_to_index: HashedMap<u32>,
	pub field_types: Vec<&'a IRType<'a>>,
	pub functions: HashedMap<IRFunction<'a>>,
	pub name: String,
	pub is_layout: bool
}

impl<'a> IRStructuredType<'a> {
	pub fn new(ctx: &'a IRContext<'a>, name: String, layout: bool, fields: Vec<(u64, &'a IRType<'a>)>) -> PositionlessResult<Self>  {
		let mut map = HashedMap::new(fields.len());
		let mut typeVec: Vec<BasicTypeEnum<'a>> = Vec::new();
		let mut field_types: Vec<&'a IRType<'a>> = Vec::new();

		let mut ind = 0;
		for entry in fields {
			map.put(entry.0, ind);

			field_types.push(entry.1);
			typeVec.push(entry.1.get_inkwell_basetype()?.into());

			ind += ind;
		}

		let inkwell_type = ctx.inkwell_ctx.struct_type(&typeVec, !layout);

		return Ok(Self { inkwell_type, field_to_index: map, name, is_layout: layout, field_types, functions: HashedMap::new(0) })
	}

	pub fn append_function(&'a mut self, hash: u64, func: IRFunction<'a>) -> PositionlessResult<bool> {
		if self.is_layout {
			return Err(PositionlessError::new("Cannot declare functions inside of a layout!"));
		}

		self.functions.put(hash, func);
		return Ok(true);
	}

	pub fn get_function(&'a self, hash: u64) -> PositionlessResult<&'a IRFunction<'a>> {
		if self.is_layout {
			return Err(PositionlessError::new("Cannot use typed-functions inside of a layout!"));
		}

		return match self.functions.get(hash) {
			Some(v) => Ok(v),
			None => Err(PositionlessError::new("Function was not founc within the struct!"))
		}
	}

	pub fn get_pointer_for_field_index(&'a self, ctx: &'a IRContext<'a>, instance: &'a IRPointer<'a>, ind: u32) -> PositionlessResult<IRPointer<'a>> {
		if ind >= self.field_types.len() as u32 {
			return Err(PositionlessError::new("Invalid index given to get_pointer_for_field_index"));
		}

		let field_ptr = match ctx.builder.build_struct_gep(self.inkwell_type, instance.inkwell_ptr, ind, "field_ptr") {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_struct_gep failed!"))
		};

		let field_type = self.field_types[ind as usize];

		return Ok(IRPointer::new(field_ptr, field_type, String::from("__inner_field_ptr")));
	}

	pub fn get_pointer_for_field(&'a self, ctx: &'a IRContext<'a>, instance: &'a IRPointer<'a>, hash: u64) -> PositionlessResult<IRPointer<'a>> {
		let k = match self.field_to_index.get(hash) {
			Some(v) => *v,
			None => return Err(PositionlessError::new(&format!("The given string hash {} doesn't represent any field in the struct {}", hash, self.name)))
		};

		return self.get_pointer_for_field_index(ctx, instance, k);
	}
} 
