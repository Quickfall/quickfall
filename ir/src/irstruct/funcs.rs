use std::{cell::RefCell, rc::Rc};

use commons::err::{PositionedError, PositionlessError, PositionlessResult};
use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module, types::BasicType, values::{BasicValueEnum, FunctionValue, IntValue}};

use crate::{ctx::{IRContext, IRLocalContext}, irstruct::ptr::IRPointer, refs::IRValueRef, types::typing::IRType, values::IRValue};

pub struct IRFunction {
	inkwell_ctx: Rc<Context>,

	pub inkwell_func: FunctionValue<'static>,
	ret_type: Option<Rc<IRType>>,
	args: Vec<Rc<IRType>>,
	name: String,

	pub lctx: IRLocalContext<'a>,

	entry: Option<BasicBlock<'static>>
}

impl IRFunction {
	pub fn new(ctx: &'a IRContext, name: String, func: FunctionValue<'a>, ret_type: Option<&'a IRType<'a>>, args: Vec<&'a IRType<'a>>) -> Self {

		let block = ctx.inkwell_ctx.append_basic_block(func, "entry");

		return IRFunction { inkwell_func: func, ret_type, args, name, entry: Some(block), lctx: IRLocalContext::new().into() }
	}

	pub fn new_shadow(name: String, func: FunctionValue<'a>, ret_type: Option<&'a IRType<'a>>, args: Vec<&'a IRType<'a>>) -> Self {
		return IRFunction { inkwell_func: func, ret_type, args, name, entry: None, lctx: IRLocalContext::new().into() }
	}

	pub fn create_shadow(ctx: &'a IRContext, name: String, module: &Module<'a>, ret_type: Option<&'a IRType<'a>>, args: Vec<&'a IRType<'a>>) -> PositionlessResult<Self> {
		let mut kargs = vec![];

		for k in &args {
			kargs.push(k.get_inkwell_base_metadatatype()?);
		}

		let t = match ret_type {
			Some(ret) => ret.get_inkwell_basetype()?.fn_type(&kargs, false),
			None => ctx.void_type.fn_type(&kargs, false)
		};

		let func = module.add_function(&name, t, None);

		return Ok(IRFunction::new_shadow(name, func, ret_type, args));
	}

	pub fn create(ctx: &'a IRContext, name: String, module: &Module<'a>, ret_type: Option<&'a IRType<'a>>, args: Vec<&'a IRType<'a>>) -> PositionlessResult<Self> {
		let mut kargs = vec![];

		for k in &args {
			kargs.push(k.get_inkwell_base_metadatatype()?);
		}

		let t = match ret_type {
			Some(ret) => ret.get_inkwell_basetype()?.fn_type(&kargs, false),
			None => ctx.void_type.fn_type(&kargs, false)
		};

		let func = module.add_function(&name, t, None);

		return Ok(IRFunction::new(ctx, name, func, ret_type, args));
	}

	pub fn call(&'a self, ctx: &'a IRContext<'a>, args: Vec<IRValueRef<'a>>, grab_return: bool) -> PositionlessResult<Option<IRPointer<'a>>> {
		let mut inkwell_args = vec![];

		for arg in args {
			inkwell_args.push(arg.obtain(ctx)?.obtain().into());
		}

		let call = match ctx.builder.build_call(self.inkwell_func, &inkwell_args, &self.name) {
			Ok(v) => v,
			Err(_) => return Err(PositionlessError::new("build_call failed!"))
		};

		if !grab_return {
			return Ok(None);
		}

		let return_type = match self.ret_type {
			Some(ret) => ret,
			None => return Ok(None)
		};

		let val = match call.try_as_basic_value().basic() {
			Some(v) => v,
			None => return Ok(None)
		};

		let val = IRValue::new(val, return_type);

		let pointer = IRPointer::create(ctx, format!("function_ret_{}", self.name), return_type, Some(IRValueRef::from_val(val)))?;

		return Ok(Some(pointer));
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