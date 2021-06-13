use crate::command::Command;
use crate::memtable::Memtable;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};

pub struct Executor {
    memtable: Arc<RwLock<Box<dyn Memtable<String, String>>>>,
    wal: Arc<RwLock<File>>,
}
impl Executor {
    pub fn new(
        memtable: Arc<RwLock<Box<dyn Memtable<String, String>>>>,
        wal: Arc<RwLock<File>>,
    ) -> Self {
        Self { memtable, wal }
    }
    pub fn execute(&mut self, command: Command) -> String {
        match command {
            Command::Set { key, body } => {
                let mut file = self.wal.write().unwrap();
                file.write(body.as_bytes());
                let body_len = body.len() as i32;
                file.write(&body_len.to_le_bytes());
                file.write(key.as_bytes());
                let key_len = key.len() as i16;
                file.write(&key_len.to_le_bytes());
                self.memtable.write().unwrap().insert(key, body);
                "STORED".to_string()
            }
            Command::Get { key } => self
                .memtable
                .read()
                .unwrap()
                .search(&key)
                .unwrap_or(&"".to_string())
                .to_string(),
            Command::Delete { key } => {
                self.memtable.write().unwrap().delete(&key);
                "DELETED".to_string()
            }
        }
    }
}
