//! AST value -> IR value conversion

use commons::err::{PositionlessError, PositionlessResult};
use inkwell::values::BasicValue;
use parser::ast::tree::ASTTreeNode;

use crate::{bools::{make_bool_cmp_int, make_bool_xor}, ctx::{IRContext, IRLocalContext}, irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable}, math::make_math_operation, refs::IRValueRef, types::{POINTER_TYPE_HASH, SIGNED64_TYPE_HASH}, values::IRValue};

pub fn get_variable_ref<'a>(lctx: &'a IRLocalContext<'a>, ctx: &'a IRContext<'a>, hash: u64) -> PositionlessResult<IRValueRef<'a>> {
	match ctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_static(IRStaticVariable::clone(v))),
		Err(_) => {}
	};

	match lctx.get_variable(hash) {
		Ok(v) => return Ok(IRValueRef::from_pointer(IRPointer::clone(v))),
		Err(_) => return Err(PositionlessError::new(&format!("Cannot find variable with hash {} in the current context", hash)))
	};
}

pub fn parse_ir_value<'a>(func: Option<&'a IRFunction<'a>>, ctx: &'a IRContext<'a>, node: Box<ASTTreeNode>, left: Option<IRPointer<'a>>) -> PositionlessResult<IRValueRef<'a>> {
	match node.as_ref() {
		ASTTreeNode::IntegerLit(v) => {
			let t = ctx.type_storage.get(SIGNED64_TYPE_HASH);

			if !t.is_some() {
				return Err(PositionlessError::new("Invalid type storage! si64 not found!"));
			}

			return Ok(IRValueRef::from_val(IRValue::from_signed(t.unwrap(), *v as i128)?));
		},

		ASTTreeNode::StringLit(v) => {
			let t = ctx.type_storage.get(POINTER_TYPE_HASH);

			if !t.is_some() {
				return Err(PositionlessError::new("Invalid type storage! pointer not found!"));
			}

			let global = IRStaticVariable::from_str(&ctx.builder, v, String::from("__string_literal"), t.unwrap())?;

			return Ok(IRValueRef::from_static(global));
		},

		ASTTreeNode::VariableReference(e) => {
			if left.as_ref().is_some() {
				let struct_t = left.as_ref().unwrap().t.get_structured_type_descriptor()?;

				let ptr = struct_t.get_pointer_for_field_noref(ctx, left.unwrap(), e.hash)?;

				return Ok(IRValueRef::from_pointer(ptr));
			}

			let var = get_variable_ref(&func.unwrap().lctx.borrow(), ctx, e.hash)?;

			return Ok(var);
		},

		ASTTreeNode::FunctionCall { func, args } => {
			let mut arguments = vec![];

			if left.as_ref().is_some() {
				arguments.push(IRValueRef::from_pointer(left.as_ref().unwrap().clone()));
			}			

			for arg in &args[0..args.len()] {
				arguments.push(parse_ir_value(lctx, ctx, arg.clone(), None)?);
			}

			let res: Option<IRPointer<'a>>;
	
			if left.is_some() {
				let descriptor = left.as_ref().unwrap().t.get_structured_type_descriptor()?;

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
			let left = parse_ir_value(lctx, ctx, lval.clone(), None)?;
			let right = parse_ir_value(lctx, ctx, rval.clone(), None)?;

			let t = left.get_type();

			let l_val = match left.obtain(ctx)?.obtain_as_int(t) {
				Some(v) => v,
				None => return Err(PositionlessError::new("lval on math operation wasn't a number!")),
			};

			let r_val = match right.obtain(ctx)?.obtain_as_int(t) {
				Some(v) => v,
				None => return Err(PositionlessError::new("lval on math operation wasn't a number!")),
			};

			let out = make_math_operation(&ctx.builder, l_val, r_val, String::from("_math"), operator.clone())?;

			if *assigns {
				if left.as_pointer().is_err() {
					return Err(PositionlessError::new("Assignments were enabled on math operation while left value wasn't a variable!"));
				}

				let ptr = left.as_pointer()?;
				ptr.store(&ctx.builder, out.as_basic_value_enum());
			}

			return Ok(IRValueRef::from_val(IRValue::new(out.into(), t)));
		},

		ASTTreeNode::OperatorBasedConditionMember { lval, rval, operator } => {
			let l_val = parse_ir_value(lctx, ctx, lval.clone(), None)?;
			let r_val = parse_ir_value(lctx, ctx, rval.clone(), None)?;

			let cmp = make_bool_cmp_int(ctx, l_val, r_val, operator.clone())?;

			return Ok(IRValueRef::from_val(cmp));
		},

		ASTTreeNode::BooleanBasedConditionMember { val, negate } => {
			let v = parse_ir_value(lctx, ctx, val.clone(), None)?;

			if *negate {
				return Ok(IRValueRef::from_val(make_bool_xor(ctx, v)?))
			}

			return Ok(v);
		}

		ASTTreeNode::StructLRFunction { l, r } => {
			let l_val = parse_ir_value(lctx, ctx, l.clone(), None)?;
			let l_ptr = l_val.as_pointer()?;
			
			return parse_ir_value(lctx, ctx, r.clone(), Some(l_ptr));
		},

		ASTTreeNode::StructLRVariable { l, r } => {
			let l_val = parse_ir_value(lctx, ctx, l.clone(), None)?;
			let l_ptr = l_val.as_pointer()?;

			return parse_ir_value(lctx, ctx, r.clone(), Some(l_ptr));
		}

		_ => return Err(PositionlessError::new("The given node cannot be parsed as a value!"))
	}
}