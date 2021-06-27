use crate::record::{decode_file, encode};
use crate::value::Value;
use anyhow::Result;
use log::info;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;

pub trait SSTable: Sync + Send {
    fn search(&self, key: &String) -> Option<Option<&Value>>;
    fn create(&mut self, records: Vec<(&String, Option<&Value>)>) -> Result<()>;
}

pub struct HashMapSSTable {
    dir: &'static Path,
    maps: VecDeque<HashMap<String, Option<Value>>>,
}

impl HashMapSSTable {
    pub fn new(dir: &'static Path) -> Result<Self> {
        let mut maps = VecDeque::new();
        let mut paths: Vec<_> = fs::read_dir(dir)?.map(|r| r.unwrap().path()).collect();
        paths.sort();
        for path in paths {
            info!("load and push_front sstable {:?}", path);
            maps.push_front(HashMap::from_iter(decode_file(path.as_ref())?))
        }
        Ok(Self { dir, maps })
    }
}

impl SSTable for HashMapSSTable {
    fn search(&self, key: &String) -> Option<Option<&Value>> {
        for map in &self.maps {
            if let Some(v) = map.get(key) {
                return Option::from(v.as_ref());
            }
        }
        return None;
    }

    fn create(&mut self, records: Vec<(&String, Option<&Value>)>) -> Result<()> {
        let file_name = format!("{:>05}.bin", self.maps.len());
        let path = self.dir.join(file_name);
        info!("create and push_front sstable {:?}", path);
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        for (key, value) in records.clone() {
            let binary = encode(key, value);
            file.write(&binary)?;
            let binary_len = binary.len() as i32;
            file.write(&binary_len.to_le_bytes())?;
        }
        self.maps.push_front(HashMap::from_iter(
            records
                .iter()
                .map(|(key, value)| ((*key).clone(), value.clone().map(|v| v.clone()))),
        ));
        Ok(())
    }
}
