use crate::avl::AvlTreeMap;
use crate::value::Value;
use crate::wal::Wal;
use anyhow::Result;
use log::info;
use std::iter::FromIterator;

pub trait Memtable: Sync + Send {
    fn insert(&mut self, key: String, value: Value) -> Result<()>;
    fn delete(&mut self, key: &String) -> Result<()>;
    fn search(&self, key: &String) -> Option<Option<&Value>>;
    fn to_vec(&self) -> Vec<(&String, &Value)>;
    fn to_records(&self) -> Vec<(&String, Option<&Value>)>;
    fn clear(&mut self) -> Result<()>;
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
    fn insert(&mut self, key: String, value: Value) -> Result<()> {
        self.wal.write(&key, Option::from(&value))?;
        self.map.insert(key, Option::from(value));
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<()> {
        self.wal.write(key, None)?;
        self.map.insert((*key).clone(), None);
        Ok(())
    }

    fn search(&self, key: &String) -> Option<Option<&Value>> {
        self.map.search(key).map(|value| value.as_ref())
    }

    fn to_vec(&self) -> Vec<(&String, &Value)> {
        self.map.iter().fold(vec![], |mut state, (key, value)| {
            if let Some(v) = value {
                state.push((key, v))
            }
            state
        })
    }

    fn to_records(&self) -> Vec<(&String, Option<&Value>)> {
        self.map
            .iter()
            .map(|(key, value)| (key, value.as_ref()))
            .collect()
    }

    fn clear(&mut self) -> Result<()> {
        self.wal.clear()?;
        self.map = AvlTreeMap::new();
        info!("clear map");
        Ok(())
    }
}
