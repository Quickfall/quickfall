//! A custom result to handle modification cleanly

#[derive(Debug)]
pub enum Maybe<K> {
    Some(K),
    None,
}

impl<K> Maybe<K> {
    /// Triggers the given code if there is a value in the Maybe block. Else panics!
    pub fn modify_if_some<F>(&mut self, f: F)
    where
        F: FnOnce(&mut K),
    {
        if let Maybe::Some(v) = self {
            f(v);
        } else {
            panic!("Used modify_if_some on Maybe::None")
        }
    }

    pub fn put(&mut self, k: K) {
        *self = Maybe::Some(k);
    }

    pub fn erase(&mut self) {
        *self = Maybe::None
    }

    pub fn unwrap(self) -> K {
        if let Maybe::Some(v) = self {
            return v;
        }

        panic!("unwrapped a none value!")
    }
}
