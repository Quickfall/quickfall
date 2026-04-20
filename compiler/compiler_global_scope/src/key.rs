use std::hash::Hash;

/// Represents a key to a global storage entry. Potentially allows for namespaces later on
#[derive(Debug)]
pub struct EntryKey {
    pub name_hash: u64,
}

impl Hash for EntryKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.name_hash);
    }
}

impl PartialEq for EntryKey {
    fn eq(&self, other: &Self) -> bool {
        self.name_hash == other.name_hash
    }
}

impl Eq for EntryKey {}
