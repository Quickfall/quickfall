//! Hashes for every generic type

use compiler_utils::hash;

pub const SIGNED_INTEGER_8: u64 = hash!("s8");
pub const SIGNED_INTEGER_16: u64 = hash!("s16");
pub const SIGNED_INTEGER_32: u64 = hash!("s32");
pub const SIGNED_INTEGER_64: u64 = hash!("s64");
pub const SIGNED_INTEGER_128: u64 = hash!("s128");
pub const SIGNED_INTEGER_ANYSIZE: u64 = hash!("s");

pub const UNSIGNED_INTEGER_8: u64 = hash!("u8");
pub const UNSIGNED_INTEGER_16: u64 = hash!("u16");
pub const UNSIGNED_INTEGER_32: u64 = hash!("u32");
pub const UNSIGNED_INTEGER_64: u64 = hash!("u64");
pub const UNSIGNED_INTEGER_128: u64 = hash!("u128");
pub const UNSIGNED_INTEGER_ANYSIZE: u64 = hash!("u");

pub const SIGNED_FLOATING_POINT_8: u64 = hash!("f8");
pub const SIGNED_FLOATING_POINT_16: u64 = hash!("f16");
pub const SIGNED_FLOATING_POINT_32: u64 = hash!("f32");
pub const SIGNED_FLOATING_POINT_64: u64 = hash!("f64");
pub const SIGNED_FLOATING_POINT_128: u64 = hash!("f128");
pub const SIGNED_FLOATING_POINT_ANYSIZE: u64 = hash!("f");

pub const SIGNED_FIXED_POINT_8: u64 = hash!("x8");
pub const SIGNED_FIXED_POINT_16: u64 = hash!("x16");
pub const SIGNED_FIXED_POINT_32: u64 = hash!("x32");
pub const SIGNED_FIXED_POINT_64: u64 = hash!("x64");
pub const SIGNED_FIXED_POINT_128: u64 = hash!("x128");
pub const SIGNED_FIXED_POINT_ANYSIZE: u64 = hash!("x");

pub const UNSIGNED_FLOATING_POINT_8: u64 = hash!("uf8");
pub const UNSIGNED_FLOATING_POINT_16: u64 = hash!("uf16");
pub const UNSIGNED_FLOATING_POINT_32: u64 = hash!("uf32");
pub const UNSIGNED_FLOATING_POINT_64: u64 = hash!("uf64");
pub const UNSIGNED_FLOATING_POINT_128: u64 = hash!("uf128");
pub const UNSIGNED_FLOATING_POINT_ANYSIZE: u64 = hash!("uf");

pub const UNSIGNED_FIXED_POINT_8: u64 = hash!("ux8");
pub const UNSIGNED_FIXED_POINT_16: u64 = hash!("ux16");
pub const UNSIGNED_FIXED_POINT_32: u64 = hash!("ux32");
pub const UNSIGNED_FIXED_POINT_64: u64 = hash!("ux64");
pub const UNSIGNED_FIXED_POINT_128: u64 = hash!("ux128");
pub const UNSIGNED_FIXED_POINT_ANYSIZE: u64 = hash!("ux");

pub const POINTER_TYPE: u64 = hash!("ptr");
pub const BOOLEAN_TYPE: u64 = hash!("bool");

pub const INTERNAL_8: u64 = hash!("internal8");
pub const INTERNAL_ANYSIZE: u64 = hash!("internal");