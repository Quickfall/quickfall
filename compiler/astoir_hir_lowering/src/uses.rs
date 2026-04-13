use std::path::{MAIN_SEPARATOR_STR};

use ast::{ctx::ParserCtx, tree::{ASTTreeNode, ASTTreeNodeKind}, types::ASTType};
use ast_parser::parse_ast_ctx;
use astoir_hir::{ctx::HIRContext, nodes::{HIRNode, HIRNodeKind}};
use compiler_typing::{raw::RawType, tree::Type};
use compiler_utils::hash::HashedString;
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, MaybeDiagnostic, builders::{make_cannot_find_type, make_diff_type_specifiers, make_req_type_kind, make_use_not_found}};
use lexer::lexer::lexer_parse_file;

use crate::{lower_ast_toplevel, structs::lower_ast_struct_declaration, types::{lower_ast_type, lower_ast_type_struct, lower_sized_base_type}, values::lower_ast_value};

pub fn handle_ast_use_statement_function_decl(context: &mut HIRContext, node: Box<ASTTreeNode>, ctx: &ParserCtx) -> MaybeDiagnostic {
	if let ASTTreeNodeKind::FunctionDeclaration { func_name, args, body: _, return_type, requires_this: _ } = node.kind.clone() {
		let ret_type;

		if return_type.is_some() {
			let lower = lower_ast_type_use_statement(context, return_type.unwrap(), &*node, ctx)?;
			
			ret_type = Some(lower)
		} else {
			ret_type = None;
		}

		let mut arguments = vec![];
		let mut types = vec![];

		for arg in args {
			types.push(arg.argument_type.clone());
			let t = lower_ast_type_use_statement(context, arg.argument_type, &*node, ctx)?;

			arguments.push((arg.name.hash, t));
		}	

		let func_name = context.functions.append(func_name.hash, (ret_type.clone(), arguments.clone(), func_name.val.clone()));
		context.function_contexts.push(None);

		// Fabricate shadow func statement to satisfy functions_declarations

		let node = HIRNode::new(HIRNodeKind::ShadowFunctionDeclaration { func_name, arguments, return_type: ret_type }, &node.start, &node.end);
		context.function_declarations.push(Some(Box::new(node))); 

		return Ok(())
	}

	panic!("Invalid node")
}

pub fn handle_ast_use_statement(context: &mut HIRContext, node: Box<ASTTreeNode>) -> MaybeDiagnostic {
	if let ASTTreeNodeKind::UseStatement { shards, use_clauses } = node.kind.clone() {
		let mut path = ".".to_string();

		for shard in shards {
			path += &MAIN_SEPARATOR_STR .to_owned();
			path += &shard.val;
		}

		path += &".qf";

		let lexer = lexer_parse_file(&path)?;
		let ast = parse_ast_ctx(&lexer)?;

		for clause in use_clauses {
			if !ast.map.contains_key(&clause.val) {
				return Err(make_use_not_found(&*node, &clause.val, &path).into())
			}

			let n = ast.map[&clause.val].clone();

			match n.kind {
				ASTTreeNodeKind::FunctionDeclaration { .. } => {
					handle_ast_use_statement_function_decl(context, n.clone(), &ast)?;
				} 
				_ => {
					lower_ast_toplevel(context, n.clone())?;
				}
			};
		}

		return Ok(())
	}

	panic!("Invalid node")
}

pub fn gather_type_use<K: DiagnosticSpanOrigin>(context: &mut HIRContext, val: HashedString, origin: &K, pass: bool, ctx: &ParserCtx) -> DiagnosticResult<RawType> {
	match context.type_storage.get_type(val.hash) {
		Ok(v) => return Ok(v),
		Err(_) => {
			if pass {
				if ctx.map.contains_key(&val.val) {
					lower_ast_struct_declaration(context, ctx.map[&val.val].clone())?;

					return gather_type_use(context, val, origin, false, ctx);
				}
			}

			return Err(make_cannot_find_type(origin, &val.val).into())
		}
	}
}

pub fn lower_ast_type_use_statement<K: DiagnosticSpanOrigin>(context: &mut HIRContext, t: ASTType, origin: &K, ctx: &ParserCtx) -> DiagnosticResult<Type> {
	return match t {
		ASTType::Generic(type_id, type_params, size_params, specifier) => {
			let val = HashedString::new(type_id.clone());
			let hash = val.hash;

			let mut t = gather_type_use(context, val, origin, true, ctx)?;

			if specifier.is_some() {
				let container = match t {
					RawType::Enum(v) => v,
					_ => return Err(make_req_type_kind(origin, &"enum".to_string()).into())
				};

				t = container.get_entry(HashedString::new(specifier.unwrap()))?
			}

			if t.get_type_params_count(&context.type_storage) != type_params.len() {
				return Err(make_diff_type_specifiers(origin, &type_params.len(), &t.get_type_params_count(&context.type_storage)).into())
			}

			let mut t_params = vec![];

			for type_param in type_params {
				t_params.push(Box::new(lower_ast_type(context, *type_param, origin)?));
			}

			let res = Type::Generic(t.clone(), t_params, size_params);
			
			if t.is_sized() {
				let lower = lower_sized_base_type(context, &res, origin)?;

				if context.type_storage.type_to_ind.contains_key(&lower) {
					return Ok(Type::Generic(t, vec![], vec![]));
				} else {
					let ind = match context.type_storage.append_with_hash(hash, lower) {
						Ok(v) => v,
						Err(_) => panic!("Generic lowering type cannot be found on type_to_hash")
					};

					return Ok(Type::Generic(context.type_storage.types.vals[ind].clone(), vec![], vec![]))
				}
			}

			return Ok(res);
		},

		ASTType::Pointer(array, inner) => Ok(Type::Pointer(array, Box::new(lower_ast_type(context, *inner, origin)?))),
		ASTType::Reference(inner) => Ok(Type::Reference(Box::new(lower_ast_type(context, *inner, origin)?))),
		ASTType::Array(size, inner) => Ok(Type::Array(size, Box::new(lower_ast_type(context, *inner, origin)?)))
	};
}
