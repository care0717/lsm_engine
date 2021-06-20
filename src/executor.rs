use crate::command::Command;
use crate::memtable::Memtable;

use crate::binary::encode;
use crate::value::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};

pub struct Executor {
    memtable: Arc<RwLock<Box<dyn Memtable<String, Value>>>>,
    wal: Arc<RwLock<File>>,
}

impl Executor {
    pub fn new(
        memtable: Arc<RwLock<Box<dyn Memtable<String, Value>>>>,
        wal: Arc<RwLock<File>>,
    ) -> Self {
        Self { memtable, wal }
    }
    pub fn execute(&mut self, command: Command) -> Result<String, Box<dyn Error + '_>> {
        match command {
            Command::Set { key, value } => {
                let mut file = self.wal.write()?;
                let mut memtable = self.memtable.write()?;
                let binary = encode(&key, Option::from(&value));
                file.write(&binary)?;
                let binary_len = binary.len() as i32;
                file.write(&binary_len.to_le_bytes())?;
                memtable.insert(key, value);
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
                let mut file = self.wal.write()?;
                let mut memtable = self.memtable.write()?;
                let binary = encode(&key, None);
                file.write(&binary)?;
                let binary_len = binary.len() as i32;
                file.write(&binary_len.to_le_bytes())?;
                memtable.delete(&key);
                Ok("DELETED".to_string())
            }
            Command::Stats {} => {
                let memtable = self.memtable.read()?;
                Ok(format!("STAT curr_items {}", memtable.to_vec().len()))
            },
        }
    }
}
