use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub trait Hashable<K: Hash> {
    fn hash(&self, item: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

    fn hash_to_index(&self, hash: u64, size: usize) -> usize {
        (hash % (size as u64)) as usize
    }
}