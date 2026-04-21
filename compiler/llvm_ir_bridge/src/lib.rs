//! The bridge between Quickfall MIR and LLVM IR.

use std::rc::Rc;

use astoir_mir::ctx::MIRContext;
use inkwell::context::Context;

use crate::{blocks::bridge_llvm_blocks, ctx::LLVMBridgeContext, funcs::bridge_llvm_functions};

pub mod blocks;
pub mod ctx;
pub mod funcs;
pub mod insts;
pub mod types;
pub mod utils;

pub fn bridge_llvm(mir: &MIRContext) -> LLVMBridgeContext {
    let ctx = Rc::new(Context::create());

    let mut ctx = LLVMBridgeContext::new(ctx);

    bridge_llvm_functions(mir, &mut ctx);
    bridge_llvm_blocks(mir, &mut ctx);

    return ctx;
}

#[macro_export]
macro_rules! llvm_to_base {
    ($exp:expr) => {
        match $exp {
            Ok(v) => v,
            Err(_) => panic!("inkwell function failed"),
        }
    };
}

#[macro_export]
macro_rules! llvm_to_base_returnless {
    ($exp:expr) => {
        match $exp {
            Ok(_) => {}
            Err(_) => panic!("inkwell function failed"),
        }
    };
}
