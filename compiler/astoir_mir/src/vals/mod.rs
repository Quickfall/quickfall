//! The definitions for every value kind in the MIR.

use crate::vals::{float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue};

pub mod arrays;
pub mod base;
pub mod consts;
pub mod float;
pub mod int;
pub mod ptr;
pub mod refer;
pub mod structs;

pub enum MIRAnyValue {
    Int(MIRIntValue),
    Float(MIRFloatValue),
    Ptr(MIRPointerValue),
}

impl From<MIRIntValue> for MIRAnyValue {
    fn from(value: MIRIntValue) -> Self {
        return MIRAnyValue::Int(value);
    }
}

impl From<MIRFloatValue> for MIRAnyValue {
    fn from(value: MIRFloatValue) -> Self {
        return MIRAnyValue::Float(value);
    }
}

impl From<MIRPointerValue> for MIRAnyValue {
    fn from(value: MIRPointerValue) -> Self {
        return MIRAnyValue::Ptr(value);
    }
}
