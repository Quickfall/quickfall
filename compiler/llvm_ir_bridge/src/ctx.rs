use std::collections::{HashMap, HashSet};
use std::{mem::transmute, rc::Rc};

use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::{context::Context, types::VoidType};

use crate::{
    types::LLVMTypeStorage,
    utils::{LLVMBasicValue, LLVMBlock, LLVMFunction},
};

pub struct LLVMBridgeContext {
    pub blocks: HashMap<usize, LLVMBlock>,
    pub values: HashMap<usize, LLVMBasicValue>,
    pub completed_blocks: HashSet<usize>,
    pub functions: HashMap<usize, LLVMFunction>,

    pub types: LLVMTypeStorage,

    pub module: Module<'static>,

    pub void_type: VoidType<'static>,

    pub ctx: Rc<Context>,
    pub builder: Builder<'static>,
}

impl LLVMBridgeContext {
    pub fn new(ctx: Rc<Context>) -> Self {
        LLVMBridgeContext {
            blocks: HashMap::new(),
            completed_blocks: HashSet::new(),
            types: LLVMTypeStorage::new(&ctx),
            functions: HashMap::new(),
            void_type: unsafe { transmute::<VoidType, VoidType<'static>>(ctx.void_type()) },
            values: HashMap::new(),
            ctx: ctx.clone(),
            builder: unsafe { transmute::<Builder, Builder<'static>>(ctx.create_builder()) },
            module: unsafe { transmute::<Module, Module<'static>>(ctx.create_module("")) },
        }
    }
}
