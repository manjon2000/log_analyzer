use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub enum TypeLogPattern {
    TextPlain,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    pub logs: HashMap<String, Log>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub name: String,
    pub regex: String,
    pub path: String,
    pub type_log: TypeLogPattern,
    pub fields: Vec<String>,
}