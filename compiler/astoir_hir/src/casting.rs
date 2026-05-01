use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{
        make_expected_simple_error, make_field_no_such, make_field_type, make_struct_missing_field,
    },
};
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
                    if !fields.contains_key(&field.0) {
                        return Err(make_struct_missing_field(origin, &t, &field.0.val).into());
                    }

                    if fields[&field.0]
                        .get_type(context, func_entry, origin)?
                        .unwrap()
                        != field.1
                    {
                        return Err(make_field_type(origin, &field.0.val, &field.1).into());
                    }
                }

                for field in &fields {
                    if !t.has_field(
                        field.0.val.clone(),
                        field.1.get_type(context, func_entry, origin)?.unwrap(),
                    ) {
                        return Err(make_field_no_such(origin, &field.0.val).into());
                    }
                }

                let mut f = vec![];

                for field in t.get_fields() {
                    f.push(fields[&field.0].clone())
                }

                return Ok(Box::new(HIRNode::new(
                    HIRNodeKind::StructInitTyped { t, fields: f },
                    &self.start,
                    &self.end,
                )));
            }

            panic!("Invalid node")
        }

        todo!("Add casting here")
    }
}
