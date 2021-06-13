pub trait Memtable<T, U> {
    fn insert(&mut self, key: T, value: U);
    fn delete(&mut self, key: &T);
    fn search(&self, key: &T) -> Option<&U>;
}
