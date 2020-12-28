pub trait Map<K, V> {
    fn add(&mut self, key: K, value: V);
    fn contains(& self, key: K) -> bool;
    fn get(&self, key: K) -> Option<&V>;
    fn size(&self) -> usize;
}
