use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeLogPattern {
    TEXT_PLAIN,
    JSON,
    NONE
}

#[derive(Debug)]
pub struct LogPattern {
    pub name: String,
    pub pattern: Regex,
    pub path: String,
    pub fields: Vec<String>,
    pub type_log: TypeLogPattern
}

impl LogPattern {
   pub fn new(
       name: &str,
       pattern: &str,
       path: &str,
       fields: Vec<String>,
       type_log: TypeLogPattern
   ) -> Result<Self, regex::Error> {
        Ok(LogPattern{
            name: name.to_string(),
            pattern: Regex::new(pattern)?,
            path: path.to_string(),
            fields: fields.into_iter().map(String::from).collect(),
            type_log
        })
    }
}