mod pattern;
mod parse;

use std::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::io::{BufRead};
use std::path::Path;
use crate::parse::parse_log;
use crate::pattern::*;

// ip, identity, userid, date, method, path, protocol, status, bytes, referer, user_agent

fn read_lines_text_plain(path: &Path) -> Result<Vec<String>, String>  {
    let read_file = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    Ok(
        read_file.lines()
            .map(|lines| lines.to_string())
            .collect::<Vec<String>>()
    )

}

fn main() {

    let log_access_apache = LogPattern::new(
        "apache",
        r#"(?P<ip>[^\s]+) (?P<identity>[^\s]+) (?P<userid>[^\s]+) \[(?P<date>[^\]]+)\] "(?P<method>[A-Z]+) (?P<path>[^\s]+) (?P<protocol>[^\s]+)" (?P<status>\d+) (?P<bytes>\d+|-) "(?P<referer>[^"]*)" "(?P<user_agent>[^"]*)""#,
        "./src/access.log",
        vec![
            "ip".to_string(),
            "identity".to_string(),
            "userid".to_string(),
            "date".to_string(),
            "method".to_string(),
            "path".to_string(),
            "protocol".to_string(),
            "status".to_string(),
            "bytes".to_string(),
            "referer".to_string(),
            "user_agent".to_string()
        ],
        TypeLogPattern::TEXT_PLAIN
    );

    let patterns = vec![
        log_access_apache,
    ];

    let mut results_fields: Vec<HashMap<String, String>> = Vec::new();
    for pattern in patterns.iter() {
        if let Ok(result) = pattern {
            match result.type_log {
                TypeLogPattern::TEXT_PLAIN =>  {
                    let lines = read_lines_text_plain(
                        Path::new(&result.path)
                    );
                    if let Ok(ref lines) = lines {
                        for log in lines.iter().cloned() {
                            let mut log_str = log.clone();
                            let result_serialize_log = parse_log(
                                &log_str,
                                result.pattern.to_string(),
                                &result.fields
                            );

                            if let Ok(serialize) = result_serialize_log {
                                results_fields.push(serialize);
                                //println!("{:?}", serialize.get("ip"));
                            }
                        }
                    }
                },
                TypeLogPattern::JSON => {},
                TypeLogPattern::NONE => {
                    println!("No specific type log");
                    break;
                }
            }
        }
    }
}