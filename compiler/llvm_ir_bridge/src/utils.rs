use std::{mem::transmute, ops::Deref, rc::Rc};

use inkwell::{
    basic_block::BasicBlock,
    context::Context,
    types::{BasicMetadataTypeEnum, BasicTypeEnum, IntType, PointerType},
    values::{BasicValueEnum, FunctionValue},
};
use rand::{Rng, distributions::Alphanumeric};

pub type LLVMBlock = LLVMSiblingObject<BasicBlock<'static>>;
pub type LLVMBasicValue = LLVMSiblingObject<BasicValueEnum<'static>>;
pub type LLVMFunction = LLVMSiblingObject<FunctionValue<'static>>;

pub type LLVMIntType = LLVMSiblingObject<IntType<'static>>;
pub type LLVMPointerType = LLVMSiblingObject<PointerType<'static>>;
pub type LLVMTypeEnum = LLVMSiblingObject<BasicTypeEnum<'static>>;
pub type LLVMMetadataEnum = LLVMSiblingObject<BasicMetadataTypeEnum<'static>>;

#[derive(Clone)]
pub struct LLVMObject<T: Clone> {
    pub inner: T,
    pub ctx: Rc<Context>,
}

/// The LLVMObject without a safety reference.
///
/// # Safety
/// Using this is only safe when the object it is contained in contains a Context ref
#[derive(Clone)]
pub struct LLVMSiblingObject<T: Clone> {
    pub inner: T,
}

impl<K: Clone> LLVMSiblingObject<K> {
    pub fn new(inner: K) -> Self {
        return LLVMSiblingObject {
            inner: unsafe { transmute(inner) },
        };
    }

    pub fn new_ref(inner: &K) -> Self {
        return LLVMSiblingObject {
            inner: unsafe { transmute(K::clone(inner)) },
        };
    }
}

impl<K: Clone> LLVMObject<K> {
    pub fn new(ctx: &Rc<Context>, inner: K) -> Self {
        return LLVMObject {
            inner: unsafe { transmute(inner) },
            ctx: ctx.clone(),
        };
    }

    pub fn new_ref(ctx: &Rc<Context>, inner: &K) -> Self {
        return LLVMObject {
            inner: unsafe { transmute(K::clone(inner)) },
            ctx: ctx.clone(),
        };
    }

    pub fn new_ownership(ctx: Rc<Context>, inner: K) -> Self {
        return LLVMObject {
            inner: unsafe { transmute(inner) },
            ctx: ctx,
        };
    }
}

impl<T: Clone> Deref for LLVMObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.inner;
    }
}

impl<T: Clone> Deref for LLVMSiblingObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.inner;
    }
}

pub fn get_block_name() -> String {
    let s = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    return s;
}
