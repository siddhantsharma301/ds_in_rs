pub trait Set<K> {
    fn add(&mut self, item: K);
    fn contains(&self, item: K) -> bool;
    fn remove(&mut self, item: K);
    fn size(&self) -> usize;
}