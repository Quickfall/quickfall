use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_expected_simple_error};
use typing::{FieldMethodType, container::Type};

use crate::{
    context::HIRContext,
    nodes::{HIRNode, HIRNodeKind},
    scope::key::EntryKey,
};

impl HIRNode {
    pub fn is_abstract(&self) -> bool {
        match self.kind {
            HIRNodeKind::StructuredInit { .. } => true,
            _ => false,
        }
    }

    pub fn use_as<K: DiagnosticSpanOrigin>(
        &self,
        context: &mut HIRContext,
        func_entry: Option<&EntryKey>,
        t: Type,
        origin: &K,
    ) -> DiagnosticResult<Box<HIRNode>> {
        let self_type = self.get_type(context, func_entry, origin)?;

        if self_type.is_none() {
            return Err(make_expected_simple_error(origin, &t, &"void".to_string()).into());
        }

        let self_type = self_type.unwrap();

        if self_type == t {
            return Ok(Box::new(self.clone()));
        }

        if self.is_abstract() {
            if let HIRNodeKind::StructuredInit { fields } = self.kind.clone() {
                for field in t.get_fields() {
					if !fields.contains_key(k)
				}
            }

            panic!("Invalid node")
        }

        todo!("Add casting here")
    }
}
