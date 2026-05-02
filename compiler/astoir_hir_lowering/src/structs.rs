use std::collections::HashMap;

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use astoir_hir::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};
use diagnostics::DiagnosticResult;

use crate::values::lower_ast_value;

pub fn lower_ast_struct_init(
    context: &mut HIRContext,
    func_key: Option<&EntryKey>,
    node: Box<ASTTreeNode>,
) -> DiagnosticResult<Box<HIRNode>> {
    if let ASTTreeNodeKind::StructInitializer { map } = node.kind.clone() {
        let mut new_map = HashMap::new();

        for entry in map {
            new_map.insert(entry.0, lower_ast_value(context, func_key, entry.1)?);
        }

        return Ok(Box::new(HIRNode::new(
            HIRNodeKind::StructuredInit { fields: new_map },
            &node.start,
            &node.end,
        )));
    }

    panic!("Invalid node!")
}
