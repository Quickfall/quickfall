use commons::err::{PositionedError, PositionlessError, PositionlessResult};
use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module, types::BasicType, values::{BasicValueEnum, FunctionValue, IntValue}};

use crate::types::typing::IRType;

pub struct IRFunction<'a> {
	pub inkwell_func: FunctionValue<'a>,
	ret_type: &'a IRType<'a>,
	args: Vec<&'a IRType<'a>>,
	name: String,

	entry: Option<BasicBlock<'a>>
}

impl<'a> IRFunction<'a> {
	pub fn new(ctx: &'a Context, name: String, func: FunctionValue<'a>, ret_type: &'a IRType<'a>, args: Vec<&'a IRType<'a>>) -> Self {

		let block = ctx.append_basic_block(func, "entry");

		return IRFunction { inkwell_func: func, ret_type, args, name, entry: Some(block) }
	}

	pub fn new_shadow(name: String, func: FunctionValue<'a>, ret_type: &'a IRType<'a>, args: Vec<&'a IRType<'a>>) -> Self {
		return IRFunction { inkwell_func: func, ret_type, args, name, entry: None }
	}

	pub fn create_shadow(name: String, module: &Module<'a>, ret_type: &'a IRType<'a>, args: Vec<&'a IRType<'a>>) -> PositionlessResult<Self> {
		let mut kargs = vec![];

		for k in &args {
			kargs.push(k.get_inkwell_base_metadatatype()?);
		}

		let t = ret_type.get_inkwell_basetype()?.fn_type(&kargs, false);

		let func = module.add_function(&name, t, None);

		return Ok(IRFunction::new_shadow(name, func, ret_type, args));
	}

	pub fn create(ctx: &'a Context, name: String, module: &Module<'a>, ret_type: &'a IRType<'a>, args: Vec<&'a IRType<'a>>) -> PositionlessResult<Self> {
		let mut kargs = vec![];

		for k in &args {
			kargs.push(k.get_inkwell_base_metadatatype()?);
		}

		let t = ret_type.get_inkwell_basetype()?.fn_type(&kargs, false);

		let func = module.add_function(&name, t, None);

		return Ok(IRFunction::new(ctx, name, func, ret_type, args));
	}

	/// Prepares the addition of the function body.
	pub fn prepare_body_filling(&self, builder: &Builder<'a>) {
		if self.entry.is_none() {
			return;
		}

		builder.position_at_end(self.entry.unwrap());
	}

	pub fn get_nth_arg(&self, ind: u32) -> PositionlessResult<BasicValueEnum<'a>> {
		let res = match self.inkwell_func.get_nth_param(ind) {
			Some(v) => v,
			None => return Err(PositionlessError::new("Couldn't get nth param using get_nth_param"))
		};

		return Ok(res);
	}

	pub fn get_nth_arg_int(&self, ind: u32) -> PositionlessResult<IntValue<'a>> {
		if !self.args[ind as usize].is_numeric_type() {
			return Err(PositionlessError::new("Tried getting nth argument but given argument's type isn't numeric!"));
		}

		return Ok(self.get_nth_arg(ind)?.into_int_value());

	}

}