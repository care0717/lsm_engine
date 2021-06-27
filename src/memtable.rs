use crate::avl::AvlTreeMap;
use crate::value::Value;
use crate::wal::Wal;
use anyhow::Result;
use std::error::Error;
use std::iter::FromIterator;

pub trait Memtable: Sync + Send {
    fn insert(&mut self, key: String, value: Value) -> Result<(), Box<dyn Error + '_>>;
    fn delete(&mut self, key: &String) -> Result<(), Box<dyn Error + '_>>;
    fn search(&self, key: &String) -> Option<&Value>;
    fn to_vec(&self) -> Vec<(&String, &Value)>;
}

pub struct AvlMemtable {
    wal: Wal,
    map: AvlTreeMap<String, Option<Value>>,
}
impl AvlMemtable {
    pub fn new(mut wal: Wal) -> Result<Self> {
        let vec = wal.recover()?;
        Ok(Self {
            wal,
            map: AvlTreeMap::from_iter(vec),
        })
    }
}
impl Memtable for AvlMemtable {
    fn insert(&mut self, key: String, value: Value) -> Result<(), Box<dyn Error + '_>> {
        self.wal.write(&key, Option::from(&value))?;
        self.map.insert(key, Option::from(value));
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<(), Box<dyn Error + '_>> {
        self.wal.write(key, None)?;
        self.map.insert((*key).clone(), None);
        Ok(())
    }

    fn search(&self, key: &String) -> Option<&Value> {
        if let Some(value) = self.map.search(key) {
            if let Some(v) = value {
                return Option::from(v);
            }
        }
        None
    }

    fn to_vec(&self) -> Vec<(&String, &Value)> {
        self.map.iter().fold(vec![], |mut state, (key, value)| {
            if let Some(v) = value {
                state.push((key, v))
            }
            state
        })
    }
}
