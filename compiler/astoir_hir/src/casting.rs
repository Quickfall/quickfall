use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{
        make_diff_type_val, make_expected_simple_error, make_field_no_such, make_field_type,
        make_struct_missing_field,
    },
    unsure_panic,
};
use typing::{FieldMethodType, TypeTransmutation, container::Type};

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

        if self_type.can_transmute(t.clone()) {
            match &self.kind {
                HIRNodeKind::IntegerLiteral(a, _) => {
                    if !t.get_raw().t.is_integer() {
                        unsure_panic!("tried making integer lit with no integer type!")
                    }

                    return Ok(self.with(HIRNodeKind::IntegerLiteral(*a, t.get_raw().t)));
                }

                HIRNodeKind::FloatLiteral(a, _) => {
                    if !t.get_raw().t.is_floating() && !t.get_raw().t.is_fixed() {
                        unsure_panic!("tried making float lit with no floating integer types!")
                    }

                    return Ok(self.with(HIRNodeKind::FloatLiteral(*a, t.get_raw().t)));
                }

                HIRNodeKind::ArrayVariableInitValue { vals } => {
                    if can_transmute_inner(&self_type, &t) {
                        let mut values = vec![];
                        let inner = t.get_next();

                        for val in vals {
                            values.push(val.use_as(context, func_entry, inner.clone(), origin)?)
                        }

                        return Ok(self.with(HIRNodeKind::ArrayVariableInitValue { vals: values }));
                    }
                }

                HIRNodeKind::ArrayVariableInitValueSame { size, val } => {
                    if can_transmute_inner(&self_type, &t) {
                        let new_val = val.use_as(context, func_entry, t.get_next(), origin)?;

                        return Ok(self.with(HIRNodeKind::ArrayVariableInitValueSame {
                            size: *size,
                            val: new_val,
                        }));
                    }
                }

                _ => {
                    return Ok(self.with(HIRNodeKind::CastValue {
                        intentional: false,
                        value: Box::new(self.clone()),
                        old_type: self_type.clone(),
                        new_type: t.clone(),
                    }));
                }
            }
        }

        return Err(make_diff_type_val(origin, &t, &self_type).into());
    }
}

pub fn can_transmute_inner(array_type: &Type, new_type: &Type) -> bool {
    if !array_type.is_array() || !new_type.is_array() {
        unsure_panic!(
            "either ones of the types sent using can_transmute_inner were not array types"
        );
    }

    return array_type.get_next().can_transmute(new_type.get_next());
}
