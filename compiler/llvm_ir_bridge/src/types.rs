use std::{collections::HashMap, mem::transmute, num::NonZero, rc::Rc};

use compiler_errors::errs::{BaseResult, base::BaseError};
use compiler_typing::{raw::RawType, tree::Type};
use inkwell::{AddressSpace, context::Context, types::{BasicType, BasicTypeEnum}};

use crate::{utils::LLVMTypeEnum};

pub struct LLVMTypeStorage {
	pub map: HashMap<RawType, LLVMTypeEnum>,

	pub ctxref: Rc<Context>
}

impl LLVMTypeStorage {
	pub fn new(ctx: &Rc<Context>) -> Self {
		LLVMTypeStorage { map: HashMap::new(), ctxref: ctx.clone() }
	}

	pub fn convert(&mut self, base: Type) -> BaseResult<LLVMTypeEnum> {
		match base {
			Type::GenericLowered(raw) => return self.convert_raw(raw),
			Type::Generic(_, _, _) => return Err(BaseError::err("Cannot lower non lowered generics!".to_string())),

			Type::Pointer(_, _) => return self.convert_raw(RawType::Pointer),

			Type::Array(size, inner) => {
				let inner_type = self.convert(*inner)?;

				Ok(LLVMTypeEnum::new(inner_type.array_type(size as u32).into()))
			}
		}
	}

	pub fn convert_raw(&mut self, base: RawType) -> BaseResult<LLVMTypeEnum> {
		if self.map.contains_key(&base) {
			return Ok(LLVMTypeEnum::clone(&self.map[&base]));
		}

		let conv: BasicTypeEnum = match &base {
			RawType::Integer(a, _) => {
				self.ctxref.custom_width_int_type(NonZero::new(*a as u32).unwrap()).unwrap().into()
			},

			RawType::Floating(a, _) => {
				match a {
					16 => self.ctxref.f16_type().into(),
					32 => self.ctxref.f32_type().into(),
					64 => self.ctxref.f64_type().into(),
					80 => self.ctxref.x86_f80_type().into(),
					128 => self.ctxref.f128_type().into(),

					_ => return Err(BaseError::err("Cannot convert to LLVM type".to_string()))
				}
			},

			RawType::FixedPoint(a, b, _) => {
				let sum = a + b;

				self.ctxref.custom_width_int_type(NonZero::new(sum as u32).unwrap()).unwrap().into()
			},

			RawType::Boolean => self.ctxref.bool_type().into(),
			
			RawType::Pointer => {
				self.ctxref.ptr_type(AddressSpace::from(0u16)).into()
			},

			_ => return Err(BaseError::err(format!("Cannot convert to LLVM type {:#?}", base.clone()).to_string()))
		};

		let l = LLVMTypeEnum::new(unsafe { transmute::<BasicTypeEnum, BasicTypeEnum<'static>>(conv) });

		self.map.insert(base, l.clone());

		return Ok(l);
	}
}