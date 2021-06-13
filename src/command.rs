use crate::command::Command::{Delete, Get, Set};
use crate::value::Value;

pub enum Command {
    Set { key: String, value: Value },
    Get { key: String },
    Delete { key: String },
}

pub fn new_command_set<'a>(key: String, value: Value) -> Command {
    return Set { key, value };
}

pub fn new_command_get(key: String) -> Command {
    return Get { key };
}

pub fn new_command_delete(key: String) -> Command {
    return Delete { key };
}
