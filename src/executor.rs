use crate::avl::AvlTreeMap;
use crate::command::Command;
use crate::memtable::Memtable;

pub struct Executor {
    memtable: Box<dyn Memtable<String, String>>,
}
impl Executor {
    pub fn new_avl() -> Self {
        Self {
            memtable: Box::new(AvlTreeMap::new()),
        }
    }
    pub fn execute(&mut self, command: Command) -> String {
        match command {
            Command::Set { key, body } => {
                self.memtable.insert(key, body);
                "STORED".to_string()
            }
            Command::Get { key } => self
                .memtable
                .search(&key)
                .unwrap_or(&"".to_string())
                .to_string(),
            Command::Delete { key } => {
                self.memtable.delete(&key);
                "DELETED".to_string()
            }
        }
    }
}
