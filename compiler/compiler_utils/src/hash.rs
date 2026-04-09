//!
//! Hash related utilities
//! 

use std::{hash::{Hash}};

pub type TypeHash = u64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashedString {
	pub val: String,
	pub hash: u64
}

impl HashedString {
	pub fn new(val: String) -> Self {
		HashedString { val: val.clone(), hash: fnv_1ahash_str(val) }
	}
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct SelfHash {
	pub hash: u64
}

impl Hash for SelfHash {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_u64(self.hash);
	}
}

pub const fn fnv_1ahash(to_hash: &str) -> u64 {
	let mut hash: u64 = 14695981039346656037;
	let bytes = to_hash.as_bytes();
	let mut i = 0;

	while i < bytes.len() {
		hash ^= bytes[i] as u64;
		hash = hash.wrapping_mul(1099511628211);
		i += 1;
	}

	return hash;
} 

pub fn fnv_1ahash_str(to_hash: String) -> u64 {
	let mut hash: u64 = 14695981039346656037;
	let bytes = to_hash.as_bytes();
	let mut i = 0;

	while i < bytes.len() {
		hash ^= bytes[i] as u64;
		hash = hash.wrapping_mul(1099511628211);
		i += 1;
	}

	return hash;
} 

#[macro_export]
macro_rules! hash {
	($expr:expr) => {
		compiler_utils::hash::fnv_1ahash($expr)
	};
}