use std::rc::Rc;

use errors::{INVALID_EXPR, IR_FIND_TYPE, errs::{CompilerResult, ErrorKind, base::BaseError, normal::CompilerError}};
use parser::ast::tree::{ASTTreeNode, ASTTreeNodeKind};

use crate::{ctx::IRContext, irstruct::structs::IRStructuredType, types::typing::IRType};

pub fn parse_ir_struct_decl(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> CompilerResult<Rc<IRType>> {
	if let ASTTreeNodeKind::StructLayoutDeclaration { name, layout, members } = node.kind.clone() {	
		let mut fields: Vec<(u64, Rc<IRType>)> = vec![];

		for member in members {
			if let ASTTreeNodeKind::StructFieldMember { name, member_type } = member.kind {
				let ir_type = match ctx.type_storage.get(member_type) {
					Some(v) => v,
					None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end))
				};

				fields.push((name.hash, ir_type));
			}
		}

		let struc = match IRStructuredType::new(ctx, name.val.clone(), layout, fields) {
			Ok(v) => v,
			Err(b) => return Err(CompilerError::from_base(b, &node.start, &node.end))
		};

		let struct_type = Rc::new(struc);
	

		if layout {
			ctx.type_storage.insert(name.hash, IRType::Layout(struct_type));
		} else {
			ctx.type_storage.insert(name.hash, IRType::Struct(struct_type));
		}

		println!("name: {} -> {}", name.val.clone(), name.hash);
		println!("opt: {}", ctx.type_storage.get(name.hash).is_none());

		let res = match ctx.type_storage.get(name.hash) {
			Some(v) => v,
			None => return Err(CompilerError::from_ast(ErrorKind::Error, IR_FIND_TYPE!().to_string(), &node.start, &node.end))
		};

		return Ok(res);
	}

	return Err(CompilerError::from_ast(ErrorKind::Critical, INVALID_EXPR!().to_string(), &node.start, &node.end))
}