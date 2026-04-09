//! AST value -> IR value conversion

use std::rc::Rc;

use errors::{FUNC_RETVOID_USE_VAL, INVALID_EXPR, IR_FIND_PRIMITIVE_TYPE, IR_FIND_TYPE, IR_FIND_VARIABLE, IR_REQ_VARIABLE_ASSIGN, NO_PERMITTED_OUTSIDE_FUNC, PARSE_VALUE, errs::{BaseResult, CompilerResult, ErrorKind, base::BaseError, normal::CompilerError}};
use inkwell::values::BasicValue;
use parser::ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::{bools::{make_bool_cmp_int, make_bool_xor}, conv::func::parse_ir_function_call, ctx::{IRContext, IRLocalContext}, irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable}, math::make_math_operation, refs::IRValueRef, types::{POINTER_TYPE_HASH, typing::OwnedValueEnum}, values::IRValue};

pub fn get_variable_ref(lctx: &IRLocalContext, ctx: &IRContext, hash: u64) -> BaseResult<IRValueRef> {
	match ctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_static(v.clone())),
		Err(_) => {}
	};

	match lctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_pointer(IRPointer::clone(v))),
		Err(_) => {}
	};

	match lctx.get_argument(hash) {
		Ok(v) => return Ok(IRValueRef::from_val(IRValue::clone(v))),
		Err(_) => return Err(BaseError::err(IR_FIND_VARIABLE!().to_string()))
	}
}

pub fn parse_ir_value<'a>(f: Option<&IRFunction>, ctx: &IRContext, node: Box<ASTTreeNode>, left: Option<IRPointer>, in_var: bool) -> CompilerResult<IRValueRef> {
	match node.as_ref().kind.clone() {
		ASTTreeNodeKind::IntegerLit { val: v, hash} => {
			let t = ctx.type_storage.get(hash);
		 
			if !t.is_some() {
				return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end));
			}

			if t.as_ref().unwrap().is_signed() {
				match IRValue::from_signed(ctx, t.as_ref().unwrap().clone(), v as i128) {
					Ok(v) => return Ok(IRValueRef::from_val(v)),
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				}
			} else {
				match IRValue::from_unsigned(ctx, t.as_ref().unwrap().clone(), v as u128) {
					Ok(v) => return Ok(IRValueRef::from_val(v)),
					Err(e) => return Err(CompilerError::from_base(e, &node.start, &node.end))
				}
			}		
		},

		ASTTreeNodeKind::StringLit(v) => {
			let t = ctx.type_storage.get(POINTER_TYPE_HASH);

			if !t.is_some() {
				return Err(CompilerError::from_ast(ErrorKind::Critical, IR_FIND_PRIMITIVE_TYPE!().to_string(), &node.start, &node.end));
			}

			
			if in_var {
				return Ok(IRValueRef::from_tempstr(v.clone()))
			}

			let global = match IRStaticVariable::from_str(&ctx, &v, String::from("__string_literal"), t.unwrap()) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			return Ok(IRValueRef::from_static(Rc::new(global)));
		},

		ASTTreeNodeKind::VariableReference(e) => {
			if left.as_ref().is_some() {
				let struct_t = match left.as_ref().unwrap().t.get_structured_type_descriptor() {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

				let ptr = match struct_t.get_pointer_for_field_noref(ctx, left.unwrap(), e.hash) {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

				return Ok(IRValueRef::from_pointer(ptr));
			}

			let var = match get_variable_ref(&f.unwrap().lctx, ctx, e.hash) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			return Ok(var);
		},

		ASTTreeNodeKind::FunctionCall { func: _, args: _ } => {

			if f.is_none() {
				return Err(CompilerError::from_ast(ErrorKind::Error, NO_PERMITTED_OUTSIDE_FUNC!().to_string(), &node.start, &node.end))
			}

			let k = parse_ir_function_call(ctx, f.unwrap(), node.clone(), left, in_var)?;

			if k.is_none() {
				return Err(CompilerError::from_ast(ErrorKind::Error, FUNC_RETVOID_USE_VAL!().to_string(), &node.start, &node.end));
			}

			return Ok(k.unwrap());
		},

		ASTTreeNodeKind::MathResult { lval, rval, operator, assigns } => {
			let left = parse_ir_value(f, ctx, lval.clone(), None, in_var)?;
			let right = parse_ir_value(f, ctx, rval.clone(), None, in_var)?;

			let t = left.get_type();

			let l_ob = match left.obtain(ctx) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			let l_val = match l_ob.obtain_as_int(ctx, t.clone()) {
				Some(v) => *v,
				None => return Err(CompilerError::from_ast(ErrorKind::Error, INVALID_EXPR!().to_string(), &node.start, &node.end)),
			};

			let r_ob = match right.obtain(ctx) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};
			
			let r_val = match r_ob.obtain_as_int(ctx, t.clone()) {
				Some(v) => *v,
				None => return Err(CompilerError::from_ast(ErrorKind::Error, INVALID_EXPR!().to_string(), &node.start, &node.end)),
			};

			let out = match make_math_operation(&ctx.builder, l_val, r_val, String::from("_math"), operator.clone()) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			if assigns {
				if left.as_pointer().is_err() {
					return Err(CompilerError::from_ast(ErrorKind::Error, IR_REQ_VARIABLE_ASSIGN!().to_string(), &node.start, &node.end));
				}

				let ptr = match left.as_pointer() {
					Ok(v) => v,
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};

				ptr.store(ctx, out.as_basic_value_enum());
			}

			return Ok(IRValueRef::from_val(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, out.into()), t)));
		},

		ASTTreeNodeKind::OperatorBasedConditionMember { lval, rval, operator } => {
			let l_val = parse_ir_value(f, ctx, lval.clone(), None, in_var)?;
			let r_val = parse_ir_value(f, ctx, rval.clone(), None, in_var)?;

			let cmp = match make_bool_cmp_int(ctx, l_val, r_val, operator.clone()) {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};


			return Ok(IRValueRef::from_val(cmp));
		},

		ASTTreeNodeKind::BooleanBasedConditionMember { val, negate } => {
			let v = parse_ir_value(f, ctx, val.clone(), None, in_var)?;

			if negate {
				match make_bool_xor(ctx, v) {
					Ok(v) => return Ok(IRValueRef::from_val(v)),
					Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
				};
			}

			return Ok(v);
		}

		ASTTreeNodeKind::StructLRFunction { l, r } => {
			let l_val = parse_ir_value(f, ctx, l.clone(), None, in_var)?;
			let l_ptr = match l_val.as_pointer() {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};
			
			return parse_ir_value(f, ctx, r.clone(), Some(l_ptr), in_var);
		},

		ASTTreeNodeKind::StructLRVariable { l, r } => {
			let l_val = parse_ir_value(f, ctx, l.clone(), None, in_var)?;
			let l_ptr = match l_val.as_pointer() {
				Ok(v) => v,
				Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
			};

			return parse_ir_value(f, ctx, r.clone(), Some(l_ptr), in_var);
		}

		_ => return Err(CompilerError::from_ast(ErrorKind::Error, PARSE_VALUE!().to_string(), &node.start, &node.end))
	}
}