pub trait Memtable<T, U>: Sync + Send {
    fn insert(&mut self, key: T, value: U);
    fn delete(&mut self, key: &T);
    fn search(&self, key: &T) -> Option<&U>;
}
