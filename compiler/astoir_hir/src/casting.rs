use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_expected_simple_error};
use typing::container::Type;

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

        todo!("Add casting here")
    }
}
