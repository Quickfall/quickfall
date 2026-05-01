//! Definitions for each node kind of HIR

use std::collections::HashMap;

use compiler_utils::{
    Position,
    hash::HashedString,
    operators::{ComparingOperator, MathOperator},
};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::make_expected_simple_error};
use typing::{
    container::Type,
    enums::ParentEnumContainer,
    raw::{InformationRawType, RawType},
    structs::StructContainer,
};

use crate::{
    PureCompTimeCandidate,
    context::{HIRContext, local::BranchedContext},
    ifelse::HIRIfBranch,
    lru::StructLRUStep,
    scope::key::EntryKey,
};

#[derive(Clone)]
pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: Position,
    pub end: Position,
}

#[derive(Clone)]
pub enum HIRNodeKind {
    IntegerLiteral(i128, RawType),
    FloatLiteral(f64, RawType),
    StringLiteral(String),

    CastValue {
        intentional: bool,
        value: Box<HIRNode>,
        old_type: Type,
        new_type: Type,
    },

    VariableDeclaration {
        variable: usize,
        name: HashedString,
        var_type: Type,
        default_val: Option<Box<HIRNode>>,
    },

    StaticVariableDeclaration {
        variable: usize,
        name: HashedString,
        var_type: Type,
        val: Box<HIRNode>,
    },

    VarAssignment {
        variable: usize,
        val: Box<HIRNode>,
    },

    MathOperation {
        left: Box<HIRNode>,
        right: Box<HIRNode>,
        operation: MathOperator,
    },

    UnwrapCondition {
        original: Box<HIRNode>,
        new_type: Type,
        new_var: Option<usize>,
        new_var_name: Option<HashedString>,
        unsafe_unwrap: bool,
    },

    UnwrapValue {
        original: Box<HIRNode>,
        new_type: Type,
        unsafe_unwrap: bool,
    },

    VariableReference {
        index: usize,
        name: HashedString,
        static_key: Option<EntryKey>,
    },

    FunctionReference {
        index: usize,
        name: HashedString,
    },

    PointerGrab {
        val: Box<HIRNode>,
    },

    ReferenceGrab {
        val: Box<HIRNode>,
    },

    StructLRU {
        steps: Vec<StructLRUStep>, // TODO: Change this to actual type
        last: Type,
    },

    EnumParentCast {
        val: Box<HIRNode>,
        parent: Type,
    },

    EnumDeclaration {
        type_name: usize,
        raw_name: HashedString,
        container: ParentEnumContainer,
    },

    StructDeclaration {
        type_name: usize,
        raw_name: HashedString,
        container: StructContainer,
        layout: bool,
    },

    StructFunctionDeclaration {
        func_name: usize,
        arguments: Vec<(HashedString, Type)>,
        return_type: Option<Type>,
        body: Vec<Box<HIRNode>>,
        ctx: &'static BranchedContext,
        requires_this: bool,
    },

    ArrayVariableInitValue {
        vals: Vec<Box<HIRNode>>,
    },

    ArrayVariableInitValueSame {
        size: usize,
        val: Box<HIRNode>,
    },

    ArrayIndexAccess {
        val: Box<HIRNode>,
        index: Box<HIRNode>,
    },

    ArrayIndexModify {
        array: Box<HIRNode>,
        index: Box<HIRNode>,
        new_val: Box<HIRNode>,
    },

    StructuredInit {
        fields: HashMap<HashedString, Box<HIRNode>>,
    },

    StructInitTyped {
        t: Type,
        fields: Vec<Box<HIRNode>>,
    },

    FunctionDeclaration {
        func_name: usize,
        raw_name: HashedString,
        arguments: Vec<(HashedString, Type)>,
        return_type: Option<Type>,
        body: Vec<Box<HIRNode>>,
        ctx: &'static BranchedContext,
        requires_this: bool,
    },

    ShadowFunctionDeclaration {
        func_name: usize,
        raw_name: HashedString,
        arguments: Vec<(HashedString, Type)>,
        return_type: Option<Type>,
    },

    FunctionCall {
        func_name: usize,
        arguments: Vec<Box<HIRNode>>,
    },

    WhileBlock {
        condition: Box<HIRNode>,
        body: Vec<Box<HIRNode>>,
    },

    ForBlock {
        initial_value: Box<HIRNode>,
        condition: Box<HIRNode>,
        incrementation: Box<HIRNode>,
        body: Vec<Box<HIRNode>>,
    },

    IfStatement {
        branches: Vec<HIRIfBranch>, // TODO: change to type
    },

    ReturnStatement {
        value: Option<Box<HIRNode>>,
    },

    BooleanOperator {
        left: Box<HIRNode>,
        right: Box<HIRNode>,
        operator: ComparingOperator,
    },

    BooleanCondition {
        value: Box<HIRNode>,
        negation: bool,
    },
}

impl HIRNode {
    pub fn new(kind: HIRNodeKind, start: &Position, end: &Position) -> Self {
        HIRNode {
            kind,
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn get_type<K: DiagnosticSpanOrigin>(
        &self,
        context: &mut HIRContext,
        func_entry: Option<&EntryKey>,
        origin: &K,
    ) -> DiagnosticResult<Option<Type>> {
        match self.kind.clone() {
            HIRNodeKind::IntegerLiteral(_, t) => Ok(Some(Type::Raw {
                raw: InformationRawType::new(t.clone()),
            })),

            HIRNodeKind::FloatLiteral(_, t) => Ok(Some(Type::Raw {
                raw: InformationRawType::new(t.clone()),
            })),

            HIRNodeKind::CastValue {
                intentional: _,
                value: _,
                old_type: _,
                new_type,
            } => Ok(Some(new_type.clone())),

            HIRNodeKind::MathOperation {
                left,
                right: _,
                operation: _,
            } => left.get_type(context, func_entry, origin),

            HIRNodeKind::UnwrapCondition {
                original: _,
                new_type,
                new_var: _,
                new_var_name: _,
                unsafe_unwrap: _,
            } => Ok(Some(new_type.clone())),

            HIRNodeKind::UnwrapValue {
                original: _,
                new_type,
                unsafe_unwrap: _,
            } => Ok(Some(new_type.clone())),

            HIRNodeKind::VariableReference {
                index,
                name: _,
                static_key,
            } => {
                if let Some(_) = static_key {
                    todo!("Add static support");
                }

                let entry = context
                    .scope
                    .get_function(func_entry.clone().unwrap(), origin)?;

                return Ok(Some(
                    entry.ctx.as_ref().clone().unwrap().variables[index]
                        .variable_type
                        .clone(),
                ));
            }

            HIRNodeKind::PointerGrab { val } => Ok(Some(Type::Pointer {
                is_array: false,
                inner: Box::new(val.get_type(context, func_entry, origin)?.unwrap()),
            })),

            HIRNodeKind::ReferenceGrab { val } => Ok(Some(Type::Pointer {
                is_array: false,
                inner: Box::new(val.get_type(context, func_entry, origin)?.unwrap()),
            })),

            HIRNodeKind::StructLRU { steps: _, last } => Ok(Some(last.clone())),

            HIRNodeKind::ArrayVariableInitValue { vals } => Ok(Some(Type::Array {
                size: vals.len(),
                inner: Box::new(vals[0].get_type(context, func_entry, origin)?.unwrap()),
            })),

            HIRNodeKind::ArrayVariableInitValueSame { size, val } => Ok(Some(Type::Array {
                size,
                inner: Box::new(val.get_type(context, func_entry, origin)?.unwrap()),
            })),

            HIRNodeKind::StructInitTyped { t, fields: _ } => Ok(Some(t.clone())),

            HIRNodeKind::FunctionCall {
                func_name,
                arguments: _,
            } => Ok(context.scope.entries[func_name]
                .as_function(origin)?
                .return_type
                .clone()),

            HIRNodeKind::BooleanCondition { .. } => Ok(Some(Type::Raw {
                raw: InformationRawType::new(RawType::Boolean),
            })),
            HIRNodeKind::BooleanOperator { .. } => Ok(Some(Type::Raw {
                raw: InformationRawType::new(RawType::Boolean),
            })),

            _ => Ok(None),
        }
    }

    pub fn with(&self, kind: HIRNodeKind) -> Box<HIRNode> {
        let mut new = self.clone();
        new.kind = kind;

        Box::new(new)
    }
}

impl PureCompTimeCandidate for HIRNodeKind {
    fn is_comptime(&self) -> bool {
        todo!()
    }

    fn is_pure(&self) -> bool {
        todo!()
    }
}
