use crate::command::Command;
use crate::memtable::Memtable;
use std::sync::{RwLock, Arc};

pub struct Executor {
    memtable: Arc<RwLock<Box<dyn Memtable<String, String>>>>,
}
impl Executor {
    pub fn new(memtable: Arc<RwLock<Box<dyn Memtable<String, String>>>>) -> Self {
        Self {
            memtable,
        }
    }
    pub fn execute(&mut self, command: Command) -> String {
        match command {
            Command::Set { key, body } => {
                self.memtable.write().unwrap().insert(key, body);
                "STORED".to_string()
            }
            Command::Get { key } => {
                self
                    .memtable.read().unwrap()
                    .search(&key)
                    .unwrap_or(&"".to_string())
                    .to_string()
            },
            Command::Delete { key } => {
                self.memtable.write().unwrap().delete(&key);
                "DELETED".to_string()
            }
        }
    }
}
