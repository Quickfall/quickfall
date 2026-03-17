use std::{collections::HashMap, mem::transmute, rc::Rc};

use astoir_typing::base::BaseType;
use compiler_errors::errs::{BaseResult, base::BaseError};
use inkwell::{context::Context, types::BasicTypeEnum};

use crate::{utils::LLVMTypeEnum};

pub struct LLVMTypeStorage {
	pub map: HashMap<BaseType, LLVMTypeEnum>,

	pub ctxref: Rc<Context>
}

impl LLVMTypeStorage {
	pub fn new(ctx: &Rc<Context>) -> Self {
		LLVMTypeStorage { map: HashMap::new(), ctxref: ctx.clone() }
	}

	pub fn convert(&mut self, base: BaseType) -> BaseResult<LLVMTypeEnum> {
		if self.map.contains_key(&base) {
			return Ok(LLVMTypeEnum::clone(&self.map[&base]));
		}

		let conv: BasicTypeEnum = match &base {
			BaseType::NumericIntegerType(a, b) => {
				self.ctxref.custom_width_int_type(*a as u32).into()
			},

			BaseType::FloatingNumberType(a, b, c) => {
				let sum = a + b;

				match sum {
					16 => self.ctxref.f16_type().into(),
					32 => self.ctxref.f32_type().into(),
					64 => self.ctxref.f64_type().into(),
					80 => self.ctxref.x86_f80_type().into(),
					128 => self.ctxref.f128_type().into(),

					_ => return Err(BaseError::err("Cannot convert to LLVM type".to_string()))
				}
			},

			BaseType::FixedPointNumberType(a, b, c) => {
				let sum = a + b;

				self.ctxref.custom_width_int_type(sum as u32).into()
			},

			BaseType::Boolean => self.ctxref.bool_type().into(),
			
			BaseType::Struct(layout, container) => {
				let mut fields = vec![];

				for field in &container.fields.vals {
					fields.push(self.convert(field.get_concrete().base.clone())?.inner)
				}

				self.ctxref.struct_type(&fields, !*layout).into()
			},

			_ => return Err(BaseError::err("Cannot convert to LLVM type".to_string()))
		};

		let l = LLVMTypeEnum::new(unsafe { transmute::<BasicTypeEnum, BasicTypeEnum<'static>>(conv) });

		self.map.insert(base, l.clone());

		return Ok(l);
	}
}