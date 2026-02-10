//!
//! The parsing module of Quickfall. Contains all of the parsing code required for the Quickfall language.
//! 
//! # Introduction
//! The `parser` module mostly contains the AST processor for Quickfall. Every element of the language is represented as an AST node which is then passed onto the AST tree. 
//! The AST tree is then sent to the IR writer to actually compile.

pub mod ast;
