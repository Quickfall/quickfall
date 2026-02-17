//! IR representation of structure types (structs, layouts...)

use commons::{err::PositionlessResult, utils::map::HashedMap};
use inkwell::{context::Context, types::{BasicTypeEnum, StructType}};

use crate::types::typing::IRType;

pub struct IRStructuredType<'a> {
	pub inkwell_type: StructType<'a>,
	pub field_to_index: HashedMap<u32>,
	pub name: String,
	pub is_layout: bool
}

impl<'a> IRStructuredType<'a> {
	pub fn new(ctx: &'a Context, name: String, layout: bool, fields: Vec<(u64, &'a IRType<'a>)>) -> PositionlessResult<Self>  {
		let mut map = HashedMap::new(fields.len());
		let mut typeVec: Vec<BasicTypeEnum<'a>> = Vec::new();

		let mut ind = 0;
		for entry in fields {
			map.put(entry.0, ind);

			typeVec.push(entry.1.get_inkwell_basetype()?.into());

			ind += ind;
		}

		let inkwell_type = ctx.struct_type(&typeVec, !layout);

		
	}
} 