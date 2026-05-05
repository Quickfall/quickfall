//!
//! AST tree related definitions.
//!

use std::{collections::HashMap, fmt::Display};

use compiler_typing::TypeParameterContainer;
use compiler_utils::{
    Position,
    hash::{HashedString, SelfHash},
    operators::{ComparingOperator, MathOperator},
};
use diagnostics::{
    DiagnosticSpanOrigin,
    diagnostic::{Diagnostic, Span, SpanKind, SpanPosition},
};

use crate::{ranges::ASTRange, types::ASTType};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclarationArgument {
    pub name: HashedString,
    pub argument_type: ASTType,
}

impl FunctionDeclarationArgument {
    pub fn new(name: String, arg_type: ASTType) -> Self {
        FunctionDeclarationArgument {
            name: HashedString::new(name),
            argument_type: arg_type,
        }
    }
}

/// The main AST node type in the AST parsing system.
#[derive(Debug, PartialEq, Clone)]
pub enum ASTTreeNodeKind {
    IntegerLit {
        val: i128,
        hash: u64,
    },
    StringLit(String),

    UseStatement {
        shards: Vec<HashedString>,
        use_clauses: Vec<HashedString>,
    },

    ThisStructParam,

    UnwrapCondition {
        original: Box<ASTTreeNode>,
        target_type: ASTType,
        unsafe_unwrap: bool,
        target_var: Option<HashedString>,
    },
    UnwrapValue {
        original: Box<ASTTreeNode>,
        target_type: ASTType,
        unsafe_unwrap: bool,
    },

    OperatorBasedConditionMember {
        lval: Box<ASTTreeNode>,
        rval: Box<ASTTreeNode>,
        operator: ComparingOperator,
    },
    BooleanBasedConditionMember {
        val: Box<ASTTreeNode>,
        negate: bool,
    },

    MathResult {
        lval: Box<ASTTreeNode>,
        rval: Box<ASTTreeNode>,
        operator: MathOperator,
    },

    VariableReference(HashedString),

    PointerGrab(Box<ASTTreeNode>),
    ReferenceGrab(Box<ASTTreeNode>),

    StructInitializer {
        map: HashMap<SelfHash, Box<ASTTreeNode>>,
    },
    ArrayVariableInitializerValueSameValue {
        size: usize,
        v: Box<ASTTreeNode>,
    },
    ArrayVariableInitializerValue {
        vals: Vec<Box<ASTTreeNode>>,
    },

    ArrayIndexAccess {
        val: Box<ASTTreeNode>,
        index: Box<ASTTreeNode>,
    },
    ArrayIndexModifiy {
        array: Box<ASTTreeNode>,
        index: Box<ASTTreeNode>,
        val: Box<ASTTreeNode>,
    },

    EnumDeclaration {
        name: HashedString,
        entries: Vec<Box<ASTTreeNode>>,
        functions: Vec<Box<ASTTreeNode>>,
        type_params: TypeParameterContainer,
    },
    EnumEntryDeclaration {
        name: HashedString,
        fields: Vec<Box<ASTTreeNode>>,
    },

    StructLayoutDeclaration {
        name: HashedString,
        layout: bool,
        members: Vec<Box<ASTTreeNode>>,
        type_params: TypeParameterContainer,
    },
    StructFieldMember {
        name: HashedString,
        member_type: ASTType,
    },

    VarDeclaration {
        var_name: HashedString,
        var_type: ASTType,
        value: Option<Box<ASTTreeNode>>,
    },
    VarValueChange {
        var: Box<ASTTreeNode>,
        value: Box<ASTTreeNode>,
    },
    VarIncrement {
        var: Box<ASTTreeNode>,
        increment_by: Option<Box<ASTTreeNode>>,
    }, // Default is by 1

    IfStatement {
        cond: Box<ASTTreeNode>,
        body: Vec<Box<ASTTreeNode>>,
        branches: Vec<Box<ASTTreeNode>>,
        depth: usize,
    },
    IfElseStatement {
        cond: Option<Box<ASTTreeNode>>,
        body: Vec<Box<ASTTreeNode>>,
    },
    ElseStatement {
        body: Vec<Box<ASTTreeNode>>,
    },

    ReturnStatement {
        val: Option<Box<ASTTreeNode>>,
    },

    StaticVariableDeclaration {
        name: HashedString,
        var_type: ASTType,
        val: Box<ASTTreeNode>,
    },

    WhileBlock {
        cond: Box<ASTTreeNode>,
        body: Vec<Box<ASTTreeNode>>,
    },

    ForBlock {
        initial_state: Box<ASTTreeNode>,
        cond: Box<ASTTreeNode>,
        increment: Box<ASTTreeNode>,
        body: Vec<Box<ASTTreeNode>>,
    },

    RangedForBlock {
        var: HashedString,
        range: ASTRange,
        body: Vec<Box<ASTTreeNode>>,
    },

    FunctionCall {
        func: HashedString,
        args: Vec<Box<ASTTreeNode>>,
    },
    FunctionDeclaration {
        func_name: HashedString,
        args: Vec<FunctionDeclarationArgument>,
        body: Vec<Box<ASTTreeNode>>,
        return_type: Option<ASTType>,
        requires_this: bool,
    },

    ExternFunctionDeclaration {
        func_name: HashedString,
        args: Vec<FunctionDeclarationArgument>,
        return_type: Option<ASTType>,
    },

    StructLRVariable {
        l: Box<ASTTreeNode>,
        r: Box<ASTTreeNode>,
    },
    StructLRFunction {
        l: Box<ASTTreeNode>,
        r: Box<ASTTreeNode>,
    },
}

impl ASTTreeNodeKind {
    pub fn is_function_call(&self) -> bool {
        return matches!(
            self,
            ASTTreeNodeKind::FunctionCall { .. } | ASTTreeNodeKind::StructLRFunction { .. }
        );
    }

    pub fn is_var_access(&self) -> bool {
        return matches!(
            self,
            ASTTreeNodeKind::VariableReference { .. } | ASTTreeNodeKind::StructLRVariable { .. }
        );
    }

    pub fn is_tree_permissible(&self) -> bool {
        return matches!(
            self,
            ASTTreeNodeKind::FunctionDeclaration { .. }
                | ASTTreeNodeKind::EnumDeclaration { .. }
                | ASTTreeNodeKind::StaticVariableDeclaration { .. }
                | ASTTreeNodeKind::ExternFunctionDeclaration { .. }
                | ASTTreeNodeKind::StructLayoutDeclaration { .. }
        );
    }

    pub fn get_tree_name(&self) -> Option<HashedString> {
        match self {
            ASTTreeNodeKind::FunctionDeclaration {
                func_name,
                args: _,
                body: _,
                return_type: _,
                requires_this: _,
            } => {
                return Some(HashedString::new(func_name.val.to_string()));
            }

            ASTTreeNodeKind::ExternFunctionDeclaration {
                func_name,
                args: _,
                return_type: _,
            } => return Some(HashedString::new(func_name.val.to_string())),

            ASTTreeNodeKind::StaticVariableDeclaration {
                name,
                var_type: _,
                val: _,
            } => {
                return Some(HashedString::new(name.val.clone()));
            }

            ASTTreeNodeKind::StructLayoutDeclaration {
                name,
                layout: _,
                members: _,
                type_params: _,
            } => {
                return Some(HashedString::new(name.val.to_string()));
            }

            ASTTreeNodeKind::VarDeclaration {
                var_name,
                var_type: _,
                value: _,
            } => {
                return Some(HashedString::new(var_name.val.to_string()));
            }

            ASTTreeNodeKind::EnumDeclaration {
                name,
                entries: _,
                functions: _,
                type_params: _,
            } => return Some(name.clone()),

            _ => return None,
        }
    }
}

/// The complete AST tree node. Contains positions and more.
#[derive(Debug, PartialEq, Clone)]
pub struct ASTTreeNode {
    pub kind: ASTTreeNodeKind,
    pub start: Position,
    pub end: Position,
}

impl ASTTreeNode {
    pub fn new(kind: ASTTreeNodeKind, start: Position, end: Position) -> Self {
        return ASTTreeNode { kind, start, end };
    }
}

impl DiagnosticSpanOrigin for ASTTreeNode {
    fn make_span(
        &self,
        kind: diagnostics::diagnostic::SpanKind,
        msg: Option<String>,
    ) -> diagnostics::diagnostic::Span {
        Span {
            start: SpanPosition::from_pos2(self.start.clone(), self.end.clone()),
            label: msg,
            kind,
        }
    }

    fn get_pos(&self) -> SpanPosition {
        SpanPosition::from_pos2(self.start.clone(), self.end.clone())
    }

    fn make_simple_diagnostic(
        &self,
        code: usize,
        level: diagnostics::diagnostic::Level,
        message: String,
        primary_span_msg: Option<String>,
        spans: Vec<Span>,
        notes: Vec<String>,
        help: Vec<String>,
    ) -> diagnostics::diagnostic::Diagnostic {
        let primary = self.make_span(SpanKind::Primary, primary_span_msg);

        Diagnostic::new_base(level, code, message, primary, spans, notes, help)
    }
}

impl Display for ASTTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;

        Ok(())
    }
}

impl Display for ASTTreeNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::UseStatement { .. } => "use statement",
            Self::UnwrapCondition { .. } | Self::UnwrapValue { .. } => "unwrap",
            Self::IntegerLit { .. } => "integer literal",
            Self::StringLit(_) => "string literal",
            Self::ThisStructParam => "this reference",
            Self::OperatorBasedConditionMember { .. } => "operator condition",
            Self::BooleanBasedConditionMember { .. } => "boolean condition",
            Self::MathResult { .. } => "math operation",
            Self::VariableReference(_) => "variable reference",
            Self::PointerGrab(_) => "pointer grabbing",
            Self::ReferenceGrab(_) => "reference",
            Self::StructInitializer { .. } => "struct value initializer",
            Self::ArrayVariableInitializerValue { .. }
            | Self::ArrayVariableInitializerValueSameValue { .. } => "array value initializer",
            Self::ArrayIndexAccess { .. } | Self::ArrayIndexModifiy { .. } => "index access",
            Self::VarDeclaration { .. } => "variable declaration",
            Self::VarValueChange { .. } => "variable assignment",
            Self::VarIncrement { .. } => "variable incrementation",
            Self::IfStatement { .. } => "if statement",
            Self::ElseStatement { .. } => "else statement",
            Self::IfElseStatement { .. } => "if else statement",
            Self::ReturnStatement { .. } => "return statement",
            Self::StaticVariableDeclaration { .. } => "static variable declaration",
            Self::WhileBlock { .. } => "while block",
            Self::ForBlock { .. } | Self::RangedForBlock { .. } => "for block",
            Self::FunctionCall { .. } => "function call",
            Self::FunctionDeclaration { .. } => "function declaration",
            Self::ExternFunctionDeclaration { .. } => "extern function declaration",
            Self::StructLRFunction { .. } => "struct LRU function usage",
            Self::StructLRVariable { .. } => "struct LRU variable usage",
            Self::StructLayoutDeclaration { .. } => "struct / layout declaration",
            Self::StructFieldMember { .. } => "struct field",
            Self::EnumDeclaration { .. } => "enum declaration",
            Self::EnumEntryDeclaration { .. } => "enum entry declaration",
        };

        write!(f, "{}", s)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! make_node {
    ($kind:expr, $s:expr, $e:expr) => {
        Box::new(ASTTreeNode::new($kind, $s.pos.clone(), $e.get_end_pos()))
    };
}
