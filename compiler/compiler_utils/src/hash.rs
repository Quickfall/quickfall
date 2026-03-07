//!
//! Hash related utilities
//! 

use core::slice;
use std::{hash::{Hash}, mem};

pub type TypeHash = u64;

///
/// Represents a value bundled with a cached hash. 
/// This is valuable for objects that are frequently compared trough hashes. 
/// This wrapper struct allows for every value change to recaculate the hash, making it safe to change.
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct WithHash<K> {
    pub val: K,
    pub hash: u64
}

impl <K> WithHash<K> {
    ///
    /// Makes a new WithHash instance with the given value.
    /// 
    /// # Examples
    /// ```
    /// let hashed: WithHash<String> = WithHash::new("abcdef".to_string());
    /// ```
    pub fn new(val: K) -> Self {
        let hash = fnv_1abyteshash(&val);

        WithHash { val, hash }
    }

    ///
    /// Changes the current value of the WithHash.
    /// 
    /// # Examples
    /// ```
    /// let mut hashed: WithHash<String> = WithHash::new("abcdef".to_string());
    /// hashed.change("my other string".to_string()); // Hash gets recalculated
    /// ```
    pub fn change(&mut self, new: K) {
		self.hash = fnv_1abyteshash(&new);
        self.val = new;
    }

    /// 
    /// Compares the value of the WithHash to another value of the same type.
    /// 
    /// # Examples
    /// ```
    /// let mut hashed: WithHash<String> = WithHash::new("abcdef".to_string());
    /// let myOtherEqualString: String = String::from("abcdef");
    /// assert!(hashed.compare(myOtherEqualString));
    /// ```
    pub fn compare(&self, val: K) -> bool {
        let hash = fnv_1abyteshash(&val);

        return self.hash == hash;
    }

    /// 
    /// Compares the stored hash of the WithHash to another hash.
    /// 
    /// # Examples
    /// ```
    /// let mut hashed: WithHash<String> = WithHash::new("abcdef".to_string());
    /// let myOtherEqualString: String = String::from("abcdef");
    /// let mut hasher = DefaultHasher::new();
    /// myOtherEqualString.hash(&mut hasher);
    /// 
    /// let hash = hasher.finish();
    /// assert!(hashed.compare_hash(hash));
    /// ```
    pub fn compare_hash(&self, hash: TypeHash) -> bool {
        return self.hash == hash;
    }

}   

#[derive(Eq, PartialEq, Debug)]
pub struct SelfHash {
	pub hash: u64
}

impl Hash for SelfHash {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_u64(self.hash);
	}
}

fn as_bytes<K>(obj: &K) -> &[u8] {
	unsafe {
		slice::from_raw_parts((obj as *const K) as *const u8, mem::size_of::<K>())
	}
}

pub fn fnv_1abyteshash<K>(to_hash: &K) -> u64 {
	let mut hash: u64 = 14695981039346656037;
	let bytes = as_bytes(to_hash);
	let mut i = 0;

	while i < bytes.len() {
		hash ^= bytes[i] as u64;
		hash = hash.wrapping_mul(1099511628211);
		i += 1;
	}

	return hash;
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

#[macro_export]
macro_rules! hash {
	($expr:expr) => {
		compiler_utils::hash::fnv_1ahash($expr)
	};
}

#[macro_export]
macro_rules! hash_k {
	($expr:expr, $t:ty) => {
		compiler_utils::hash::fnv_1abyteshash::<$t>($expr)
	};
}

