use crate::command::Command;
use crate::memtable::Memtable;
use crate::value::Value;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::error::Error;

pub struct Executor {
    memtable: Arc<RwLock<Box<dyn Memtable<String, Value>>>>,
    wal: Arc<RwLock<File>>,
}

const TOMBSTONE: i32 = -1;
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
                let value_bytes = value.as_bytes();
                file.write(&*value_bytes)?;
                let value_len = value_bytes.len() as i32;
                file.write(&value_len.to_le_bytes())?;
                file.write(key.as_bytes())?;
                let key_len = key.len() as i16;
                file.write(&key_len.to_le_bytes())?;
                self.memtable.write()?.insert(key, value);
                Ok("STORED".to_string())
            }
            Command::Get { key } => {
                let formatted_value = self
                    .memtable
                    .read()?
                    .search(&key)
                    .map_or(String::new(), |v| v.format(key));

                Ok(format!("{}END", formatted_value))
            }
            Command::Delete { key } => {
                let mut file = self.wal.write()?;
                file.write(&TOMBSTONE.to_le_bytes())?;
                file.write(key.as_bytes())?;
                let key_len = key.len() as i16;
                file.write(&key_len.to_le_bytes())?;
                self.memtable.write()?.delete(&key);
                Ok("DELETED".to_string())
            }
        }
    }
}
