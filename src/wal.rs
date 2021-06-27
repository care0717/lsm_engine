use crate::record::{decode_file, encode};
use crate::value::Value;
use anyhow::Result;
use log::info;
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct Wal {
    path: &'static Path,
    write_file: File,
}

impl Wal {
    pub fn new(path: &'static Path) -> Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            path,
            write_file: file,
        })
    }

    pub fn write(&mut self, key: &String, value: Option<&Value>) -> Result<()> {
        let binary = encode(key, value);
        self.write_file.write(&binary)?;
        let binary_len = binary.len() as i32;
        self.write_file.write(&binary_len.to_le_bytes())?;
        Ok(())
    }
    pub fn recover(&mut self) -> Result<Vec<(String, Option<Value>)>> {
        info!("recover from wal {}", self.path.to_str().unwrap());
        decode_file(self.path)
    }
    pub fn clear(&mut self) -> Result<()> {
        remove_file(self.path)?;
        self.write_file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(self.path)?;
        info!("clear wal {}", self.path.to_str().unwrap());
        Ok(())
    }
}
