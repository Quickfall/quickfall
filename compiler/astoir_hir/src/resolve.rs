//! Used to resolve incomplete HIR nodes like initializers

use compiler_typing::tree::Type;
use compiler_utils::hash::SelfHash;
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::{make_req_type_kind, make_struct_init_missing_field}};

use crate::{ctx::{HIRBranchedContext, HIRContext}, nodes::{HIRNode, HIRNodeKind}};

/// Resolves incomplete HIR nodes into complete nodes
pub fn resolve_to_type<K: DiagnosticSpanOrigin>(node: Box<HIRNode>, destination: Type, context: &HIRContext, curr_ctx: &HIRBranchedContext, origin: &K) -> DiagnosticResult<Box<HIRNode>> {
	match node.kind {
		HIRNodeKind::StructInitializer { fields } => {
			let generic = destination.as_generic_safe(origin)?;
			let mut new_fields = vec![];

			if !generic.is_field_based() {
				return Err(make_req_type_kind(origin, &"field-having".to_string()).into())
			}

			for field in destination.get_fields(&context.type_storage) {
				let identity = SelfHash { hash: field };

				if !fields.contains_key(&identity) {
					return Err(make_struct_init_missing_field(origin, &destination, &field).into())
				}

				let val = fields[&identity].clone();

				let field_data = destination.get_field(&context.type_storage, field)?.1.resolve(&destination);

				let val = Box::new(val.use_as(context, curr_ctx, field_data, origin, None)?);

				new_fields.push(val);
			}

			return Ok(Box::new(HIRNode::new(HIRNodeKind::StructInitializerTyped { t: destination, fields: new_fields }, &node.start, &node.end)))
		},

		_ => panic!("Invalid node")
	}
} 