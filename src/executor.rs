use crate::command::Command;
use crate::memtable::Memtable;

use crate::sstable::SSTable;
use std::error::Error;
use std::sync::{Arc, RwLock};

pub struct Executor {
    memtable: Arc<RwLock<Box<dyn Memtable>>>,
    sstable: Arc<RwLock<Box<dyn SSTable>>>,
}

impl Executor {
    pub fn new(
        memtable: Arc<RwLock<Box<dyn Memtable>>>,
        sstable: Arc<RwLock<Box<dyn SSTable>>>,
    ) -> Self {
        Self { memtable, sstable }
    }
    pub fn execute(&mut self, command: Command) -> Result<String, Box<dyn Error + '_>> {
        match command {
            Command::Set { key, value } => {
                let mut memtable = self.memtable.write()?;
                memtable.insert(key, value)?;
                let records = memtable.to_records();
                if records.len() > 10 {
                    let mut sstable = self.sstable.write()?;
                    sstable.create(records).unwrap();
                    memtable.clear().unwrap();
                }
                Ok("STORED".to_string())
            }
            Command::Get { key } => {
                let formatted_value = self
                    .memtable
                    .read()?
                    .search(&key)
                    .or(self.sstable.read()?.search(&key))
                    .flatten()
                    .map_or(String::new(), |v| v.to_string(key));
                Ok(format!("{}END", formatted_value))
            }
            Command::Delete { key } => {
                let mut memtable = self.memtable.write()?;
                memtable.delete(&key)?;
                Ok("DELETED".to_string())
            }
            Command::Stats {} => {
                let memtable = self.memtable.read()?;
                Ok(format!("STAT curr_items {}", memtable.to_vec().len()))
            }
        }
    }
}
