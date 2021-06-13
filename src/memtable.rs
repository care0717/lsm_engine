pub trait Memtable<T, U> {
    fn insert(
        &mut self, key: T, value: U);
    fn delete(self, key: &T) -> Self
    where
        Self: Sized;
    fn search(&self, key: &T) -> Option<&U>;
}
