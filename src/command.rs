use crate::command::Command::{Delete, Get, Set, Stats};
use crate::value::Value;

pub enum Command {
    Set { key: String, value: Value },
    Get { key: String },
    Delete { key: String },
    Stats {},
}

impl Command {
    pub fn new_set(key: String, value: Value) -> Self {
        Set { key, value }
    }
    pub fn new_get(key: String) -> Self {
        Get { key }
    }
    pub fn new_delete(key: String) -> Self {
        Delete { key }
    }
    pub fn new_stats() -> Self {
        Stats {}
    }
}
