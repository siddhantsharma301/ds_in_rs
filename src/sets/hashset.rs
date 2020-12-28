use crate::traits::set::Set;
use crate::traits::hashable::Hashable;
use crate::traits::key::Key;


pub struct HashSet<K> {
    buckets: Vec<Vec<K>>,
    size: usize,
    load_factor: f64,
}

impl<K: Key> HashSet<K> {
    pub fn new() -> Self {
        HashSet {
            buckets: vec![vec![]; 8],
            size: 0,
            load_factor: 1.5,
        }
    }

    fn resize(&mut self) {
        let new_buckets_len = self.buckets.len() * 2;
        let mut new_buckets = vec![vec![]; new_buckets_len];
        for bucket in 0 .. self.buckets.len() {
            for i in 0 .. self.buckets[bucket].len() {
                let hash = self.hash(&self.buckets[bucket][i]);
                let index = self.hash_to_index(hash, new_buckets_len);
                new_buckets[index].push(self.buckets[bucket][i]);
            }
        }
        self.buckets = new_buckets;
    }
}

impl<K: Key> Hashable<K> for HashSet<K> {}

impl<K: Key> Set<K> for HashSet<K> {
    fn add(&mut self, item: K) {
        let curr_load = (self.size as f64) / (self.buckets.len() as f64);
        if curr_load >= self.load_factor {
            self.resize();
        }

        if !self.contains(item) {
            let hash = self.hash(&item);
            let bucket = self.hash_to_index(hash, self.buckets.len());
            self.buckets[bucket].push(item);
            self.size += 1;
        }
    }

    fn contains(&self, item: K) -> bool {
        let hash = self.hash(&item);
        let bucket = self.hash_to_index(hash, self.buckets.len());
        for i in &self.buckets[bucket] {
            if i == &item {
                return true;
            }
        }
        return false;
    }

    fn remove(&mut self, item: K) {
        if self.contains(item) {
            let hash = self.hash(&item);
            let bucket = self.hash_to_index(hash, self.buckets.len());
            for i in 0 .. self.buckets[bucket].len() {
                if &item == &self.buckets[bucket][i] {
                    self.buckets[bucket].remove(i);
                    self.size -= 1;
                    break;
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}
 