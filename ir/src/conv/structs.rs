use std::rc::Rc;

use commons::err::{PositionlessError, PositionlessResult};
use parser::ast::tree::ASTTreeNode;

use crate::{ctx::IRContext, irstruct::structs::IRStructuredType, types::typing::IRType};

pub fn parse_ir_struct_decl(ctx: &mut IRContext, node: Box<ASTTreeNode>) -> PositionlessResult<Rc<IRType>> {
	if let ASTTreeNode::StructLayoutDeclaration { name, layout, members } = *node.clone() {	
		let mut fields: Vec<(u64, Rc<IRType>)> = vec![];

		for member in members {
			if let ASTTreeNode::StructFieldMember { name, member_type } = *member {
				let ir_type = match ctx.type_storage.get(member_type) {
					Some(v) => v,
					None => return Err(PositionlessError::new(&format!("Cannot find type with hash {}", member_type)))
				};

				fields.push((name.hash, ir_type));
			}
		}

		let struct_type = Rc::new(IRStructuredType::new(ctx, name.val.clone(), layout, fields)?);
	

		if layout {
			ctx.type_storage.insert(name.hash, IRType::Layout(struct_type));
		} else {
			ctx.type_storage.insert(name.hash, IRType::Struct(struct_type));
		}

		println!("name: {} -> {}", name.val.clone(), name.hash);
		println!("opt: {}", ctx.type_storage.get(name.hash).is_none());

		let res = match ctx.type_storage.get(name.hash) {
			Some(v) => v,
			None => return Err(PositionlessError::new("Cannot find newly registered struct type!"))
		};

		return Ok(res);
	}

	return Err(PositionlessError::new("Cannot parse as struct declaration as this node isn't a type decl!"))
}