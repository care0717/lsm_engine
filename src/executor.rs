use crate::memtable::Memtable;
use crate::avl::AvlTreeMap;
use crate::command::Command;

pub struct Executor {
    memtable: Box<dyn Memtable<String, String>>,
}
impl Executor {
    pub fn new_avl() -> Self {
        Self { memtable: Box::new(AvlTreeMap::new()) }
    }
    pub fn execute(&mut self, command: Command) -> String {
        match command {
            Command::Set {key, body } => {
                self.memtable.insert(key, body);
                "OK".to_string()
            }
            Command::Get { key } => {
                self.memtable.search(&key).unwrap_or(&"".to_string()).to_string()
            }
        }
    }

}
