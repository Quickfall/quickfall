use crate::scope::ScopeStorage;

pub mod local;

pub struct HIRContext {
    pub scope: ScopeStorage,
}

impl HIRContext {
    pub fn new() -> Self {
        HIRContext {
            scope: ScopeStorage::new(),
        }
    }
}
