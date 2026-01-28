//!
//! Hash related utilities
//! 

use core::hash;
use std::{any::TypeId, hash::{DefaultHasher, Hash, Hasher}};

pub type TypeHash = u64;

///
/// Represents a value bundled with a cached hash. 
/// This is valuable for objects that are frequently compared trough hashes. 
/// This wrapper struct allows for every value change to recaculate the hash, making it safe to change.
/// 
#[derive(Debug)]
pub struct WithHash<K: Hash> {
    pub val: K,
    hash: u64
}

impl <K: Hash> WithHash<K> {
    ///
    /// Makes a new WithHash instance with the given value.
    /// 
    /// # Examples
    /// ```
    /// let hashed: WithHash<String> = WithHash::new("abcdef".to_string());
    /// ```
    pub fn new(val: K) -> Self {
        let hash = utils_get_hash(&val);

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
        self.val = new;
        self.hash = utils_get_hash(&self.val);
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
        let hash = utils_get_hash(&val);

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

#[inline(always)]
fn utils_get_hash<K: Hash>(val: &K) -> TypeHash {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}   