use crate::command::Command;
use crate::memtable::Memtable;

use std::error::Error;
use std::sync::{Arc, RwLock};

pub struct Executor {
    memtable: Arc<RwLock<Box<dyn Memtable>>>,
}

impl Executor {
    pub fn new(memtable: Arc<RwLock<Box<dyn Memtable>>>) -> Self {
        Self { memtable }
    }
    pub fn execute(&mut self, command: Command) -> Result<String, Box<dyn Error + '_>> {
        match command {
            Command::Set { key, value } => {
                let mut memtable = self.memtable.write()?;
                memtable.insert(key, value).unwrap();
                Ok("STORED".to_string())
            }
            Command::Get { key } => {
                let formatted_value = self
                    .memtable
                    .read()?
                    .search(&key)
                    .map_or(String::new(), |v| v.to_string(key));

                Ok(format!("{}END", formatted_value))
            }
            Command::Delete { key } => {
                let mut memtable = self.memtable.write()?;
                memtable.delete(&key).unwrap();
                Ok("DELETED".to_string())
            }
            Command::Stats {} => {
                let memtable = self.memtable.read()?;
                Ok(format!("STAT curr_items {}", memtable.to_vec().len()))
            }
        }
    }
}
