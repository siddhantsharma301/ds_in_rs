use crate::traits::map::Map;
use crate::traits::hashable::Hashable;
use crate::traits::key::Key;


pub struct HashMap<K, V> {
    buckets: Vec<Vec<Node<K, V>>>,
    size: usize,
    load_factor: f64,
}

impl<K: Key, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: (0..8).map(|_| vec![]).collect(),
            size: 0,
            load_factor: 1.5,
        }
    }

    fn resize(&mut self) {
        if self.load_factor < (self.size / self.buckets.len()) as f64 {
            let new_size = self.buckets.len() * 2;
            let mut new_buckets : Vec<Vec<Node<K, V>>> = (0..new_size).map(|_| vec![]).collect();
            for bucket in 0 .. self.buckets.len() {
                let mut b = self.buckets.remove(bucket);
                for item in 0 .. b.len() {
                    let i = b.remove(item);
                    let hash = self.hash(&i.key);
                    let index = self.hash_to_index(hash, new_size);
                    new_buckets[index].push(i);
                }
            }
            self.buckets = new_buckets;
        }
    }
}

impl<K: Key, V> Hashable<K> for HashMap<K, V> {}

impl<K: Key, V> Map<K, V> for HashMap<K, V> {
    fn add(&mut self, key: K, value: V) {
        self.resize();
        let hash = self.hash(&key);
        let bucket = self.hash_to_index(hash, self.buckets.len());
        self.buckets[bucket].push(Node::new(key, value));
        self.size += 1;
    }

    fn contains(&self, key: K) -> bool {
        let hash = self.hash(&key);
        let bucket = self.hash_to_index(hash, self.buckets.len());
        for i in &self.buckets[bucket] {
            if i.key == key {
                return true;
            }
        }
        return false;
    }

    fn get(&self, key: K) -> Option<&V> {
        let hash = self.hash(&key);
        let bucket = self.hash_to_index(hash, self.buckets.len());
        for i in &self.buckets[bucket] {
            if i.key == key {
                return Some(&i.value);
            }
        }
        None
    }

    fn size(&self) -> usize {
        self.size
    }
}


#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
}

impl<K, V> Node<K, V> {
    fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            value: v,
        }
    }
}
