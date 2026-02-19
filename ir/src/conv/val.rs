//! AST value -> IR value conversion

use std::rc::Rc;

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::values::BasicValue;
use parser::ast::tree::ASTTreeNode;

use crate::{bools::{make_bool_cmp_int, make_bool_xor}, ctx::{IRContext, IRLocalContext}, irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable}, math::make_math_operation, refs::IRValueRef, types::{POINTER_TYPE_HASH, SIGNED64_TYPE_HASH, typing::OwnedValueEnum}, values::IRValue};

pub fn get_variable_ref(lctx: &IRLocalContext, ctx: &IRContext, hash: u64) -> PositionlessResult<IRValueRef> {
	match ctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_static(v.clone())),
		Err(_) => {}
	};

	match lctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_pointer(IRPointer::clone(v))),
		Err(_) => return Err(PositionlessError::new(&format!("Cannot find variable with hash {} in the current context", hash)))
	};
}

pub fn parse_ir_value<'a>(lctx: Option<&IRLocalContext>, ctx: &IRContext, node: Box<ASTTreeNode>, left: Option<IRPointer>, in_var_decl: bool) -> PositionlessResult<IRValueRef> {
	match node.as_ref() {
		ASTTreeNode::IntegerLit(v) => {
			let t = ctx.type_storage.get(SIGNED64_TYPE_HASH);

			if !t.is_some() {
				return Err(PositionlessError::new("Invalid type storage! si64 not found!"));
			}

			return Ok(IRValueRef::from_val(IRValue::from_signed(ctx, t.unwrap(), *v as i128)?));
		},

		ASTTreeNode::StringLit(v) => {
			let t = ctx.type_storage.get(POINTER_TYPE_HASH);

			if !t.is_some() {
				return Err(PositionlessError::new("Invalid type storage! pointer not found!"));
			}

			
			if in_var_decl {
				return Ok(IRValueRef::from_tempstr(v.clone()))
			}

			let global = IRStaticVariable::from_str(&ctx, v, String::from("__string_literal"), t.unwrap())?;

			return Ok(IRValueRef::from_static(Rc::new(global)));
		},

		ASTTreeNode::VariableReference(e) => {
			if left.as_ref().is_some() {
				let struct_t = left.as_ref().unwrap().t.get_structured_type_descriptor()?;

				let ptr = struct_t.get_pointer_for_field_noref(ctx, left.unwrap(), e.hash)?;

				return Ok(IRValueRef::from_pointer(ptr));
			}

			let var = get_variable_ref(&lctx.unwrap(), ctx, e.hash)?;

			return Ok(var);
		},

		ASTTreeNode::FunctionCall { func, args } => {
			let mut arguments = vec![];

			if left.as_ref().is_some() {
				arguments.push(IRValueRef::from_pointer(left.as_ref().unwrap().clone()));
			}			

			for arg in &args[0..args.len()] {
				arguments.push(parse_ir_value(lctx, ctx, arg.clone(), None, in_var_decl)?);
			}

			let res: Option<IRPointer>;
	
			if left.is_some() {
				let t = left.as_ref().unwrap().t.clone();
				let descriptor = t.get_structured_type_descriptor()?;

				let f = descriptor.get_function(func.hash)?;

				res = f.call(ctx, arguments, true)?;
			} else {
				let f = ctx.get_funtion(func.hash)?;

				res = f.call(ctx, arguments, true)?;
			}

			if res.is_none() {
				return Err(PositionlessError::new(&format!("Cannot use the result of function {} as a value as it is void!", func.val)));
			}

			return Ok(IRValueRef::from_pointer(res.unwrap()));
		},

		ASTTreeNode::MathResult { lval, rval, operator, assigns } => {
			let left = parse_ir_value(lctx, ctx, lval.clone(), None, in_var_decl)?;
			let right = parse_ir_value(lctx, ctx, rval.clone(), None, in_var_decl)?;

			let t = left.get_type();

			let l_val = match left.obtain(ctx)?.obtain_as_int(ctx, t.clone()) {
				Some(v) => *v,
				None => return Err(PositionlessError::new("lval on math operation wasn't a number!")),
			};

			let r_val = match right.obtain(ctx)?.obtain_as_int(ctx, t.clone()) {
				Some(v) => *v,
				None => return Err(PositionlessError::new("lval on math operation wasn't a number!")),
			};

			let out = make_math_operation(&ctx.builder, l_val, r_val, String::from("_math"), operator.clone())?;

			if *assigns {
				if left.as_pointer().is_err() {
					return Err(PositionlessError::new("Assignments were enabled on math operation while left value wasn't a variable!"));
				}

				let ptr = left.as_pointer()?;
				ptr.store(ctx, out.as_basic_value_enum());
			}

			return Ok(IRValueRef::from_val(IRValue::new(OwnedValueEnum::new(&ctx.inkwell_ctx, out.into()), t)));
		},

		ASTTreeNode::OperatorBasedConditionMember { lval, rval, operator } => {
			let l_val = parse_ir_value(lctx, ctx, lval.clone(), None, in_var_decl)?;
			let r_val = parse_ir_value(lctx, ctx, rval.clone(), None, in_var_decl)?;

			let cmp = make_bool_cmp_int(ctx, l_val, r_val, operator.clone())?;

			return Ok(IRValueRef::from_val(cmp));
		},

		ASTTreeNode::BooleanBasedConditionMember { val, negate } => {
			let v = parse_ir_value(lctx, ctx, val.clone(), None, in_var_decl)?;

			if *negate {
				return Ok(IRValueRef::from_val(make_bool_xor(ctx, v)?))
			}

			return Ok(v);
		}

		ASTTreeNode::StructLRFunction { l, r } => {
			let l_val = parse_ir_value(lctx, ctx, l.clone(), None, in_var_decl)?;
			let l_ptr = l_val.as_pointer()?;
			
			return parse_ir_value(lctx, ctx, r.clone(), Some(l_ptr), in_var_decl);
		},

		ASTTreeNode::StructLRVariable { l, r } => {
			let l_val = parse_ir_value(lctx, ctx, l.clone(), None, in_var_decl)?;
			let l_ptr = l_val.as_pointer()?;

			return parse_ir_value(lctx, ctx, r.clone(), Some(l_ptr), in_var_decl);
		}

		_ => return Err(PositionlessError::new("The given node cannot be parsed as a value!"))
	}
}