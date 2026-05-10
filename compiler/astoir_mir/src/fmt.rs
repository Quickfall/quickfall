use std::fmt::Display;

use crate::{DisplayAstoIR, ctx::MIRContext};

pub struct DisplayWithCtx<'a, K: DisplayAstoIR> {
    value: &'a K,
    ctx: &'a MIRContext,
}

impl<'a, K: DisplayAstoIR> DisplayWithCtx<'a, K> {
    pub fn new(ctx: &'a MIRContext, value: &'a K) -> Self {
        Self { value, ctx }
    }
}

impl<'a, K: DisplayAstoIR> Display for DisplayWithCtx<'a, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.format(f, self.ctx)
    }
}
