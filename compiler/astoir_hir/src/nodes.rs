//! Definitions for each node kind of HIR

use std::{collections::HashMap, fmt::Binary};

use compiler_utils::{
    Position,
    hash::HashedString,
    operators::{ComparingOperator, MathOperator},
};
use typing::{container::Type, enums::ParentEnumContainer, structs::StructContainer};

use crate::PureCompTimeCandidate;

pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: Position,
    pub end: Position,
}

pub enum HIRNodeKind {
    IntegerLiteral(i128),
    FloatLiteral(f64),

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
        is_static: bool,
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
        steps: Vec<Binary>, // TODO: Change this to actual type
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
        ctx: Binary, // TODO: add actual type here
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
        ctx: Binary, // TODO: change to type
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
        branches: Vec<Binary>, // TODO: change to type
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
}

impl PureCompTimeCandidate for HIRNodeKind {
    fn is_comptime(&self) -> bool {
        todo!()
    }

    fn is_pure(&self) -> bool {
        todo!()
    }
}
