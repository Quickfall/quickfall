use std::{cell::RefCell, mem::transmute, ops::{Deref, DerefMut}, rc::Rc};

use errors::{INKWELL_FUNC_FAILED, errs::{BaseResult, base::BaseError}};
use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module, types::BasicType, values::{BasicValueEnum, FunctionValue, IntValue}};

use crate::{ctx::{IRContext, IRLocalContext}, irstruct::ptr::IRPointer, refs::IRValueRef, types::typing::{IRType, OwnedIntValue, OwnedValueEnum}, values::IRValue};


pub struct IRFunction {
	pub owned: Rc<Context>,

	pub inkwell_func: FunctionValue<'static>,
	pub ret_type: Option<Rc<IRType>>,
	pub args: Vec<(Rc<IRType>, u64)>,
	name: String,
	pub hash: u64,

	pub lctx: IRLocalContext,

	entry: Option<BasicBlock<'static>>
}

impl IRFunction {
	pub fn new(ctx: &IRContext, name: String, hash: u64, func: FunctionValue, ret_type: Option<Rc<IRType>>, args: Vec<(Rc<IRType>, u64)>) -> Self {

		let block = ctx.inkwell_ctx.append_basic_block(func, "entry");

		return IRFunction { owned: ctx.inkwell_ctx.clone(), inkwell_func: unsafe { transmute(func)}, ret_type, args, name, hash, entry: Some(unsafe { transmute(block) }), lctx: IRLocalContext::new().into() }
	}

	pub fn new_shadow(ctx: &IRContext, name: String, hash: u64, func: FunctionValue, ret_type: Option<Rc<IRType>>, args: Vec<(Rc<IRType>, u64)>) -> Self {
		return IRFunction { owned: ctx.inkwell_ctx.clone(), inkwell_func: unsafe { transmute(func)}, ret_type, args, name, hash, entry: None, lctx: IRLocalContext::new().into() }
	}

	pub fn create_shadow(ctx: &IRContext, name: String, hash: u64, module: &Module, ret_type: Option<Rc<IRType>>, args: Vec<(Rc<IRType>, u64)>) -> BaseResult<Self> {
		let mut kargs = vec![];

		for k in &args {
			kargs.push(*k.0.get_inkwell_base_metadatatype()?);
		}

		let t = match &ret_type {
			Some(ret) => ret.get_inkwell_basetype()?.fn_type(&kargs, false),
			None => ctx.void_type.fn_type(&kargs, false)
		};

		let func = module.add_function(&name, t, None);

		return Ok(IRFunction::new_shadow(ctx, name, hash, func, ret_type, args));
	}

	pub fn create(ctx: &IRContext, name: String, hash: u64, module: &Module, ret_type: Option<Rc<IRType>>, args: Vec<(Rc<IRType>, u64)>) -> BaseResult<Self> {
		let mut kargs = vec![];

		for k in &args {
			kargs.push(*k.0.get_inkwell_base_metadatatype()?);
		}

		let t = match &ret_type {
			Some(ret) => ret.get_inkwell_basetype()?.fn_type(&kargs, false),
			None => ctx.void_type.fn_type(&kargs, false)
		};

		let func = module.add_function(&name, t, None);

		return Ok(IRFunction::new(ctx, name, hash, func, ret_type, args));
	}

	pub fn call(&self, ctx: &IRContext, args: Vec<IRValueRef>, grab_return: bool) -> BaseResult<Option<IRValue>> {
		let mut inkwell_args = vec![];

		for arg in args {
			inkwell_args.push(arg.obtain(ctx)?.obtain().inner.into());
		}

		let call = match ctx.builder.build_call(self.inkwell_func, &inkwell_args, &self.name) {
			Ok(v) => v,
			Err(e) => return Err(BaseError::critical(format!(INKWELL_FUNC_FAILED!(), "build_call", e)))
		};

		if !grab_return {
			return Ok(None);
		}

		let return_type = match self.ret_type.as_ref() {
			Some(ret) => ret.clone(),
			None => return Ok(None)
		};

		let val = match call.try_as_basic_value().basic() {
			Some(v) => v,
			None => return Ok(None)
		};

		let val = IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, val), return_type.clone());

		return Ok(Some(val));
	}

	/// Prepares the addition of the function body.
	pub fn prepare_body_filling(&self, ctx: &IRContext) {
		if self.entry.is_none() {
			return;
		}

		ctx.builder.position_at_end(self.entry.unwrap());
	}

	pub fn get_nth_arg(&self, ind: u32) -> BaseResult<OwnedValueEnum> {
		let res = match self.inkwell_func.get_nth_param(ind) {
			Some(v) => v,
			None => return Err(BaseError::critical("Couldn't get nth param using get_nth_param".to_string()))
		};

		return Ok(OwnedValueEnum::new(&self.owned, res));
	}

	pub fn get_nth_arg_int(&self, ind: u32) -> BaseResult<OwnedIntValue> {
		if !self.args[ind as usize].0.is_numeric_type() {
			return Err(BaseError::critical("Tried getting nth argument but given argument's type isn't numeric!".to_string()));
		}

		return Ok(OwnedIntValue::new(&self.owned, self.get_nth_arg(ind)?.into_int_value()));

	}

}