use std::{collections::HashMap, mem::transmute, num::NonZero, rc::Rc};

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

	pub fn convert(&mut self, base: Type) -> LLVMTypeEnum {
		match base {
			Type::GenericLowered(raw) => return self.convert_raw(raw),
			Type::Generic(_, _, _) => panic!("cannot convert unlowered generics"),

			Type::Reference(_) => return self.convert_raw(RawType::Pointer),
			Type::Pointer(_, _) => return self.convert_raw(RawType::Pointer),

			Type::Array(size, inner) => {
				let inner_type = self.convert(*inner);

				LLVMTypeEnum::new(inner_type.array_type(size as u32).into())
			}
		}
	}

	pub fn convert_raw(&mut self, base: RawType) -> LLVMTypeEnum {
		if self.map.contains_key(&base) {
			return LLVMTypeEnum::clone(&self.map[&base])
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

					_ => panic!("cannot convert float to LLVM type")
				}
			},
			
			RawType::LoweredStruct(layout, b) => {
				let mut fields = vec![];

				for field in &b.fields.vals {
					fields.push(self.convert(field.clone()).inner);
				}

				self.ctxref.struct_type(&fields, !*layout).into()
			}

			RawType::FixedPoint(a, b, _) => {
				let sum = a + b;

				self.ctxref.custom_width_int_type(NonZero::new(sum as u32).unwrap()).unwrap().into()
			},

			RawType::Boolean => self.ctxref.bool_type().into(),
			
			RawType::Pointer => {
				self.ctxref.ptr_type(AddressSpace::from(0u16)).into()
			},

			_ => panic!("cannot convert to LLVM type!")
		};

		let l = LLVMTypeEnum::new(unsafe { transmute::<BasicTypeEnum, BasicTypeEnum<'static>>(conv) });

		self.map.insert(base, l.clone());

		return l
	}
}