use crate::command::Command::{Get, Set};

pub enum Command {
    Set { key: String, body: String },
    Get { key: String },
}

pub fn new_command_set<'a>(key: String, body: String) -> Command {
    return Set { key, body };
}

pub fn new_command_get(key: String) -> Command {
    return Get { key };
}
