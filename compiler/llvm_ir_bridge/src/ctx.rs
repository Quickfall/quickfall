use std::collections::HashMap;
use std::{mem::transmute, rc::Rc};

use inkwell::module::Module;
use inkwell::{context::Context, types::VoidType};
use inkwell::builder::Builder;

use crate::{types::LLVMTypeStorage, utils::{LLVMBasicValue, LLVMBlock, LLVMFunction}};

pub struct LLVMBridgeContext {
	pub blocks: HashMap<usize, LLVMBlock>,
	pub values: Vec<LLVMBasicValue>,
	pub functions: Vec<LLVMFunction>,

	pub types: LLVMTypeStorage,

	pub module: Module<'static>,

	pub void_type: VoidType<'static>,

	pub ctx: Rc<Context>,
	pub builder: Builder<'static>

}

impl LLVMBridgeContext {
	pub fn new(ctx: Rc<Context>) -> Self {
		LLVMBridgeContext {
			blocks: HashMap::new(),
			types: LLVMTypeStorage::new(&ctx),
			functions: vec![],
			void_type: unsafe { transmute::<VoidType, VoidType<'static>>(ctx.void_type())},
			values: vec![],
			ctx: ctx.clone(),
			builder: unsafe { transmute::<Builder, Builder<'static>>(ctx.create_builder())},
			module: unsafe { transmute::<Module, Module<'static>>(ctx.create_module("__qfmirbridge__")) }
		}
	}
}