use astoir_mir::{
    blocks::MIRBlockHeldInstruction,
    ctx::MIRContext,
    insts::MIRInstruction,
    vals::{base::BaseMIRValue, float::MIRFloatValue, int::MIRIntValue, ptr::MIRPointerValue},
};
use compiler_typing::raw::RawType;
use inkwell::{
    IntPredicate,
    module::Linkage,
    types::{BasicType, BasicTypeEnum, StringRadix},
    values::{BasicValue, BasicValueEnum, FastMathFlags, FloatValue, IntValue},
};

use crate::{ctx::LLVMBridgeContext, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_instruction(
    instruction: MIRBlockHeldInstruction,
    func: usize,
    bridge: &mut LLVMBridgeContext,
    mir: &MIRContext,
) -> Option<LLVMBasicValue> {
    let res: Option<BasicValueEnum<'static>> =
        match MIRInstruction::from(instruction.clone().into()) {
            MIRInstruction::StackAlloc { alloc_size: _, t } => {
                let res = llvm_to_base!(
                    bridge
                        .builder
                        .build_alloca(bridge.types.convert(t).inner, "")
                );

                Some(res.into())
            }

            MIRInstruction::Load { value } => {
                let base = BaseMIRValue::from(value.into());

                let res = llvm_to_base!(
                    bridge.builder.build_load(
                        bridge
                            .types
                            .convert(mir.ssa_hints.get_hint(base.get_ssa_index()).get_type())
                            .inner,
                        bridge.values[&base.get_ssa_index()].into_pointer_value(),
                        &format!("{}", instruction.as_valuedindex())
                    )
                );

                Some(res.into())
            }

            MIRInstruction::Store { variable, value } => {
                let base = BaseMIRValue::from(variable.into());
                let ptr = bridge.values[&base.get_ssa_index()].into_pointer_value();

                let val = bridge.values[&value.get_ssa_index()].inner;

                llvm_to_base_returnless!(bridge.builder.build_store(ptr, val));

                None
            }

            MIRInstruction::IntegerAdd {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRIntValue::into(left);
                let right: BaseMIRValue = MIRIntValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res = llvm_to_base!(bridge.builder.build_int_add(
                    l.into_int_value(),
                    r.into_int_value(),
                    ""
                ));

                if fast {
                    let res2 = res.as_instruction_value().unwrap();

                    llvm_to_base_returnless!(res2.set_no_signed_wrap_flag(true));
                    llvm_to_base_returnless!(res2.set_no_unsigned_wrap_flag(true));
                }

                Some(res.into())
            }

            MIRInstruction::IntegerSub {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRIntValue::into(left);
                let right: BaseMIRValue = MIRIntValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static> = llvm_to_base!(bridge.builder.build_int_sub(
                    l.into_int_value(),
                    r.into_int_value(),
                    ""
                ));

                if fast {
                    let res2 = res.as_instruction_value().unwrap();

                    llvm_to_base_returnless!(res2.set_no_signed_wrap_flag(true));
                    llvm_to_base_returnless!(res2.set_no_unsigned_wrap_flag(true));
                }

                Some(res.into())
            }

            MIRInstruction::IntegerMul {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRIntValue::into(left);
                let right: BaseMIRValue = MIRIntValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static> = llvm_to_base!(bridge.builder.build_int_mul(
                    l.into_int_value(),
                    r.into_int_value(),
                    ""
                ));

                if fast {
                    let res2 = res.as_instruction_value().unwrap();

                    llvm_to_base_returnless!(res2.set_no_signed_wrap_flag(true));
                    llvm_to_base_returnless!(res2.set_no_unsigned_wrap_flag(true));
                }

                Some(res.into())
            }

            MIRInstruction::IntegerDiv {
                signed,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRIntValue::into(left);
                let right: BaseMIRValue = MIRIntValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static>;

                if signed {
                    res = llvm_to_base!(bridge.builder.build_int_signed_div(
                        l.into_int_value(),
                        r.into_int_value(),
                        ""
                    ))
                } else {
                    res = llvm_to_base!(bridge.builder.build_int_unsigned_div(
                        l.into_int_value(),
                        r.into_int_value(),
                        ""
                    ))
                }

                if fast {
                    let res2 = res.as_instruction_value().unwrap();

                    llvm_to_base_returnless!(res2.set_no_signed_wrap_flag(true));
                    llvm_to_base_returnless!(res2.set_no_unsigned_wrap_flag(true));
                    llvm_to_base_returnless!(res2.set_exact_flag(true));
                }
                Some(res.into())
            }

            MIRInstruction::IntegerMod {
                signed,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRIntValue::into(left);
                let right: BaseMIRValue = MIRIntValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static>;

                if signed {
                    res = llvm_to_base!(bridge.builder.build_int_signed_rem(
                        l.into_int_value(),
                        r.into_int_value(),
                        ""
                    ))
                } else {
                    res = llvm_to_base!(bridge.builder.build_int_unsigned_rem(
                        l.into_int_value(),
                        r.into_int_value(),
                        ""
                    ))
                }

                if fast {
                    llvm_to_base_returnless!(
                        res.as_instruction_value()
                            .unwrap()
                            .set_fast_math_flags(FastMathFlags::all())
                    )
                }

                Some(res.into())
            }

            MIRInstruction::FloatAdd {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRFloatValue::into(left);
                let right: BaseMIRValue = MIRFloatValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: FloatValue<'static> = llvm_to_base!(bridge.builder.build_float_add(
                    l.into_float_value(),
                    r.into_float_value(),
                    ""
                ));

                if fast {
                    llvm_to_base_returnless!(
                        res.as_instruction_value()
                            .unwrap()
                            .set_fast_math_flags(FastMathFlags::all())
                    )
                }

                Some(res.into())
            }

            MIRInstruction::FloatSub {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRFloatValue::into(left);
                let right: BaseMIRValue = MIRFloatValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: FloatValue<'static> = llvm_to_base!(bridge.builder.build_float_sub(
                    l.into_float_value(),
                    r.into_float_value(),
                    ""
                ));

                if fast {
                    llvm_to_base_returnless!(
                        res.as_instruction_value()
                            .unwrap()
                            .set_fast_math_flags(FastMathFlags::all())
                    )
                }

                Some(res.into())
            }

            MIRInstruction::FloatMul {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRFloatValue::into(left);
                let right: BaseMIRValue = MIRFloatValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: FloatValue<'static> = llvm_to_base!(bridge.builder.build_float_mul(
                    l.into_float_value(),
                    r.into_float_value(),
                    ""
                ));

                if fast {
                    llvm_to_base_returnless!(
                        res.as_instruction_value()
                            .unwrap()
                            .set_fast_math_flags(FastMathFlags::all())
                    )
                }

                Some(res.into())
            }

            MIRInstruction::FloatDiv {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRFloatValue::into(left);
                let right: BaseMIRValue = MIRFloatValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: FloatValue<'static> = llvm_to_base!(bridge.builder.build_float_div(
                    l.into_float_value(),
                    r.into_float_value(),
                    ""
                ));

                if fast {
                    llvm_to_base_returnless!(
                        res.as_instruction_value()
                            .unwrap()
                            .set_fast_math_flags(FastMathFlags::all())
                    )
                }

                Some(res.into())
            }

            MIRInstruction::FloatMod {
                signed: _,
                fast,
                left,
                right,
            } => {
                let left: BaseMIRValue = MIRFloatValue::into(left);
                let right: BaseMIRValue = MIRFloatValue::into(right);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: FloatValue<'static> = llvm_to_base!(bridge.builder.build_float_rem(
                    l.into_float_value(),
                    r.into_float_value(),
                    ""
                ));

                if fast {
                    llvm_to_base_returnless!(
                        res.as_instruction_value()
                            .unwrap()
                            .set_fast_math_flags(FastMathFlags::all())
                    )
                }

                Some(res.into())
            }

            MIRInstruction::BitwiseAnd { a, b } => {
                let left: BaseMIRValue = MIRIntValue::into(a);
                let right: BaseMIRValue = MIRIntValue::into(b);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static> = llvm_to_base!(bridge.builder.build_and(
                    l.into_int_value(),
                    r.into_int_value(),
                    ""
                ));

                Some(res.into())
            }

            MIRInstruction::BitwiseOr { a, b } => {
                let left: BaseMIRValue = MIRIntValue::into(a);
                let right: BaseMIRValue = MIRIntValue::into(b);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static> = llvm_to_base!(bridge.builder.build_or(
                    l.into_int_value(),
                    r.into_int_value(),
                    ""
                ));

                Some(res.into())
            }

            MIRInstruction::BitwiseXor { a, b } => {
                let left: BaseMIRValue = MIRIntValue::into(a);
                let right: BaseMIRValue = MIRIntValue::into(b);

                let l = bridge.values[&left.get_ssa_index()].clone();
                let r = bridge.values[&right.get_ssa_index()].clone();

                let res: IntValue<'static> = llvm_to_base!(bridge.builder.build_xor(
                    l.into_int_value(),
                    r.into_int_value(),
                    ""
                ));

                Some(res.into())
            }

            MIRInstruction::BitwiseNot { val } => {
                let val: BaseMIRValue = MIRIntValue::into(val);

                let v = bridge.values[&val.get_ssa_index()].clone();

                let res: IntValue<'static> =
                    llvm_to_base!(bridge.builder.build_not(v.into_int_value(), "e"));

                Some(res.into())
            }

            MIRInstruction::ShiftLeft { a, shift } => {
                let val: BaseMIRValue = MIRIntValue::into(a);
                let shift: BaseMIRValue = MIRIntValue::into(shift);

                let v = bridge.values[&val.get_ssa_index()].clone();
                let shift = bridge.values[&shift.get_ssa_index()].clone();

                let res: IntValue<'static> = llvm_to_base!(bridge.builder.build_left_shift(
                    v.into_int_value(),
                    shift.into_int_value(),
                    ""
                ));

                Some(res.into())
            }

            // TODO: add toggle between artithmetic & logical
            MIRInstruction::ShiftRight { a, shift } => {
                let signed = a.signed;

                let val: BaseMIRValue = MIRIntValue::into(a);
                let shift: BaseMIRValue = MIRIntValue::into(shift);

                let v = bridge.values[&val.get_ssa_index()].clone();
                let shift = bridge.values[&shift.get_ssa_index()].clone();

                let res = llvm_to_base!(bridge.builder.build_right_shift(
                    v.into_int_value(),
                    shift.into_int_value(),
                    signed,
                    ""
                ));

                Some(res.into())
            }

            MIRInstruction::CompEq { a, b } => {
                Some(bridge_llvm_int_cmp(a, b, IntPredicate::EQ, bridge).into())
            }
            MIRInstruction::CompNeg { a, b } => {
                Some(bridge_llvm_int_cmp(a, b, IntPredicate::EQ, bridge).into())
            }
            MIRInstruction::CompLt { a, b } => {
                if a.signed {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::SLT, bridge).into())
                } else {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::ULT, bridge).into())
                }
            }

            MIRInstruction::CompLe { a, b } => {
                if a.signed {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::SLE, bridge).into())
                } else {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::ULE, bridge).into())
                }
            }

            MIRInstruction::CompGt { a, b } => {
                if a.signed {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::SGT, bridge).into())
                } else {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::UGT, bridge).into())
                }
            }

            MIRInstruction::CompGe { a, b } => {
                if a.signed {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::SLE, bridge).into())
                } else {
                    Some(bridge_llvm_int_cmp(a, b, IntPredicate::ULE, bridge).into())
                }
            }

            MIRInstruction::IntegerSignedConstant { raw, bitsize } => {
                let t = RawType::Integer(bitsize, true);

                let int_type = bridge.types.convert_raw(t).into_int_type();
                let res = int_type
                    .const_int_from_string(&raw.to_string(), StringRadix::Decimal)
                    .unwrap();

                Some(res.into())
            }

            MIRInstruction::IntegerUnsignedConstant { raw, bitsize } => {
                let t = RawType::Integer(bitsize, false);

                let int_type = bridge.types.convert_raw(t).into_int_type();
                let res = int_type
                    .const_int_from_string(&raw.to_string(), StringRadix::Decimal)
                    .unwrap();

                Some(res.into())
            }

            MIRInstruction::FloatSignedConstant { raw, size } => {
                let t = RawType::Floating(size, true);

                let float_type = bridge.types.convert_raw(t).into_float_type();

                let res = unsafe { float_type.const_float_from_string(&raw.to_string()) };

                Some(res.into())
            }

            MIRInstruction::FloatUnsignedConstant { raw, size } => {
                let t = RawType::Floating(size, false);

                let float_type = bridge.types.convert_raw(t).into_float_type();

                let res = unsafe { float_type.const_float_from_string(&raw.to_string()) };

                Some(res.into())
            }

            MIRInstruction::FixedSignedConstant { .. }
            | MIRInstruction::FixedUnsignedConstant { .. } => {
                panic!("fixed points numbers are not currently supported")
            }

            MIRInstruction::StaticStringConstant { raw } => {
                let bytes = raw.as_bytes();
                let byte_type = bridge
                    .types
                    .convert_raw(RawType::Integer(8, false))
                    .into_int_type();

                let arr_type = byte_type.array_type((bytes.len() + 1) as u32);

                let global = bridge.module.add_global(arr_type, None, "");

                global.set_linkage(Linkage::Private);
                global.set_constant(true);
                global.set_unnamed_addr(true);

                let mut vals: Vec<IntValue> = bytes
                    .iter()
                    .map(|b| byte_type.const_int(*b as u64, false))
                    .collect();

                vals.push(byte_type.const_zero());

                global.set_initializer(&byte_type.const_array(&vals));

                Some(global.as_pointer_value().into())
            }

            MIRInstruction::StructInitializerConstant {
                struct_type,
                values,
            } => {
                let t = bridge.types.convert_raw(struct_type).into_struct_type();

                let mut vals = vec![];

                for value in values {
                    vals.push(bridge.values[&value.get_ssa_index()].clone().inner);
                }

                let val = t.const_named_struct(&vals).into();

                Some(val)
            }

            MIRInstruction::ArrayInitializerConstant { values } => {
                let k = bridge.types.convert(values[0].vtype.clone());

                let ke = match k.as_basic_type_enum() {
                    BasicTypeEnum::IntType(v) => {
                        let mut vals = vec![];

                        for value in values {
                            vals.push(bridge.values[&value.get_ssa_index()].into_int_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::FloatType(v) => {
                        let mut vals = vec![];

                        for value in values {
                            vals.push(bridge.values[&value.get_ssa_index()].into_float_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::ArrayType(v) => {
                        let mut vals = vec![];

                        for value in values {
                            vals.push(bridge.values[&value.get_ssa_index()].into_array_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::PointerType(v) => {
                        let mut vals = vec![];

                        for value in values {
                            vals.push(bridge.values[&value.get_ssa_index()].into_pointer_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::StructType(v) => {
                        let mut vals = vec![];

                        for value in values {
                            vals.push(bridge.values[&value.get_ssa_index()].into_struct_value())
                        }

                        v.const_array(&vals)
                    }

                    _ => panic!("got invalid LLVM type"),
                };

                Some(ke.into())
            }

            MIRInstruction::ArrayInitializerConstantSame { size, val } => {
                let k = bridge.types.convert(val.vtype.clone());

                let ke = match k.as_basic_type_enum() {
                    BasicTypeEnum::IntType(v) => {
                        let mut vals = vec![];

                        for _ in 0..size {
                            vals.push(bridge.values[&val.get_ssa_index()].into_int_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::FloatType(v) => {
                        let mut vals = vec![];

                        for _ in 0..size {
                            vals.push(bridge.values[&val.get_ssa_index()].into_float_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::StructType(v) => {
                        let mut vals = vec![];

                        for _ in 0..size {
                            vals.push(bridge.values[&val.get_ssa_index()].into_struct_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::ArrayType(v) => {
                        let mut vals = vec![];

                        for _ in 0..size {
                            vals.push(bridge.values[&val.get_ssa_index()].into_array_value())
                        }

                        v.const_array(&vals)
                    }

                    BasicTypeEnum::PointerType(v) => {
                        let mut vals = vec![];

                        for _ in 0..size {
                            vals.push(bridge.values[&val.get_ssa_index()].into_pointer_value())
                        }

                        v.const_array(&vals)
                    }

                    _ => panic!("got invalid LLVM type"),
                };

                Some(ke.into())
            }

            MIRInstruction::Return { val } => {
                if val.is_some() {
                    let v = bridge.values[&val.unwrap().get_ssa_index()].clone();

                    llvm_to_base_returnless!(bridge.builder.build_return(Some(&v.inner)));
                } else {
                    llvm_to_base_returnless!(bridge.builder.build_return(None));
                }

                None
            }

            MIRInstruction::UnconditionalBranch { branch } => {
                let block = bridge.blocks[&branch].clone();

                llvm_to_base_returnless!(bridge.builder.build_unconditional_branch(block.inner));

                None
            }

            MIRInstruction::ConditionalBranch {
                cond,
                if_branch,
                else_branch,
            } => {
                let cond: BaseMIRValue = MIRIntValue::into(cond);

                let cond = bridge.values[&cond.get_ssa_index()]
                    .clone()
                    .into_int_value();

                let if_branch = bridge.blocks[&if_branch].clone();
                let else_branch = bridge.blocks[&else_branch].clone();

                llvm_to_base_returnless!(bridge.builder.build_conditional_branch(
                    cond,
                    if_branch.inner,
                    else_branch.inner
                ));

                None
            }

            MIRInstruction::Phi { choices } => {
                let mut llvm_choices = vec![];

                let t = bridge.types.convert(choices[0].1.vtype.clone());

                for choice in choices {
                    let block = bridge.blocks[&choice.0].clone().inner;
                    let value = bridge.values[&choice.1.get_ssa_index()]
                        .inner
                        .as_basic_value_enum();

                    llvm_choices.push((value, block));
                }

                let phi = llvm_to_base!(bridge.builder.build_phi(t.inner, ""));

                for choice in llvm_choices {
                    phi.add_incoming(&[(&choice.0 as &dyn BasicValue, choice.1)]);
                }

                Some(phi.as_basic_value())
            }

            MIRInstruction::Select {
                cond,
                if_val,
                else_val,
            } => {
                let cond: BaseMIRValue = MIRIntValue::into(cond);

                let cond = bridge.values[&cond.get_ssa_index()]
                    .clone()
                    .into_int_value();

                let if_val = bridge.values[&if_val.get_ssa_index()].inner;
                let else_val = bridge.values[&else_val.get_ssa_index()].inner;

                let res = llvm_to_base!(bridge.builder.build_select(cond, if_val, else_val, ""));

                Some(res)
            }

            MIRInstruction::FieldPointer { val, field } => {
                let val: BaseMIRValue = MIRPointerValue::into(val);
                let struct_type = bridge
                    .types
                    .convert(mir.ssa_hints.get_hint(val.get_ssa_index()).get_type())
                    .inner;

                let ptr_val = bridge.values[&val.get_ssa_index()].inner;

                let res = llvm_to_base!(bridge.builder.build_struct_gep(
                    struct_type,
                    ptr_val.into_pointer_value(),
                    field as u32,
                    ""
                ));

                Some(res.into())
            }

            MIRInstruction::IndexPointer { val, index } => {
                let val: BaseMIRValue = MIRPointerValue::into(val);
                let struct_type = bridge
                    .types
                    .convert(mir.ssa_hints.get_hint(val.get_ssa_index()).get_type())
                    .inner;

                let index_type = bridge
                    .types
                    .convert_raw(RawType::Integer(32, false))
                    .inner
                    .into_int_type();

                let ptr_val = bridge.values[&val.get_ssa_index()].inner;

                let index: BaseMIRValue = MIRIntValue::into(index);
                let index = bridge.values[&index.get_ssa_index()].inner;

                let res = llvm_to_base!(unsafe {
                    bridge.builder.build_in_bounds_gep(
                        struct_type,
                        ptr_val.into_pointer_value(),
                        &[index_type.const_int(0, false), index.into_int_value()],
                        "",
                    )
                });

                Some(res.into())
            }

            MIRInstruction::PointerAdd { pointer, right } => {
                let pointer: BaseMIRValue = MIRPointerValue::into(pointer);
                let right: BaseMIRValue = MIRIntValue::into(right);
                let t = bridge.types.convert_raw(RawType::Integer(8, false)).inner;

                let pointer = bridge.values[&pointer.get_ssa_index()].inner;
                let right = bridge.values[&right.get_ssa_index()].inner;

                let res = llvm_to_base!(unsafe {
                    bridge.builder.build_in_bounds_gep(
                        t,
                        pointer.into_pointer_value(),
                        &[right.into_int_value()],
                        "",
                    )
                });

                Some(res.into())
            }

            MIRInstruction::PointerSub { pointer, right } => {
                let pointer: BaseMIRValue = MIRPointerValue::into(pointer);
                let right: BaseMIRValue = MIRIntValue::into(right);
                let t = bridge.types.convert_raw(RawType::Integer(8, false)).inner;

                let pointer = bridge.values[&pointer.get_ssa_index()].inner;
                let right = bridge.values[&right.get_ssa_index()].inner;

                let res = llvm_to_base!(unsafe {
                    bridge.builder.build_in_bounds_gep(
                        t,
                        pointer.into_pointer_value(),
                        &[right.into_int_value()],
                        "",
                    )
                });

                Some(res.into())
            }

            MIRInstruction::Call {
                function,
                arguments,
            } => {
                let func = bridge.functions[&function].clone().inner;

                let mut args = vec![];

                for arg in arguments {
                    args.push(bridge.values[&arg.get_ssa_index()].inner.into());
                }

                let res = llvm_to_base!(bridge.builder.build_call(func, &args, ""));

                res.try_as_basic_value().basic()
            }

            MIRInstruction::FuncArgumentGrab { ind, argtype: _ } => {
                let func = bridge.functions[&func].clone().inner;

                func.get_nth_param(ind as u32)
            }

            MIRInstruction::MemoryCopy { src, dest, sz } => {
                let src: BaseMIRValue = src.into();
                let dest: BaseMIRValue = dest.into();

                let llvm_src = bridge.values[&src.get_ssa_index()].clone();
                let llvm_dest = bridge.values[&dest.get_ssa_index()].clone();

                let sz_type = bridge
                    .types
                    .convert_raw(RawType::Integer(32, false))
                    .into_int_type();
                let sz = sz_type.const_int(sz as u64, false);

                llvm_to_base_returnless!(bridge.builder.build_memcpy(
                    llvm_dest.inner.into_pointer_value(),
                    1,
                    llvm_src.into_pointer_value(),
                    1,
                    sz
                ));

                None
            }

            MIRInstruction::IRCast { val, to: _ } => {
                Some(bridge.values[&val.get_ssa_index()].inner.clone())
            }

            _ => None,
        };

    if res.is_some() {
        return Some(LLVMBasicValue::new(res.unwrap()));
    }

    return None;
}

pub fn bridge_llvm_int_cmp(
    a: MIRIntValue,
    b: MIRIntValue,
    predicate: IntPredicate,
    bridge: &mut LLVMBridgeContext,
) -> IntValue<'static> {
    let left: BaseMIRValue = MIRIntValue::into(a);
    let right: BaseMIRValue = MIRIntValue::into(b);

    let l = bridge.values[&left.get_ssa_index()].clone();
    let r = bridge.values[&right.get_ssa_index()].clone();

    return llvm_to_base!(bridge.builder.build_int_compare(
        predicate,
        l.into_int_value(),
        r.into_int_value(),
        "e"
    ));
}
