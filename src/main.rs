mod pattern;
mod parse;
mod filter;
mod file;

use serde::{Deserialize, Serialize};

use std::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;
use crate::file::read_lines_text_plain;
use crate::parse::*;
use crate::pattern::*;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let file_config = File::open("./config.log.json")
        .expect("Error open file");
    let reader = BufReader::new(file_config);
    let config: LogConfig = serde_json::from_reader(reader)?;


    let mut timestamp_logs: Vec<SystemTime> = Vec::new();

    for (key, log) in config.logs.iter() {
        let metadata = fs::metadata(Path::new(&log.path));
        if let Ok(metadata) = metadata {
            let modified = metadata.modified();
            if let Ok(modified) = modified {
                timestamp_logs.push(modified);
            }
        }
    }

    loop {
        for (index, log) in config.logs.iter().enumerate() {
            let metadata = fs::metadata(Path::new(&log.1.path));
            if let Ok(ref metadata) = metadata {
                if let Ok(modified_time) = metadata.modified() {
                    if modified_time > timestamp_logs[index] {
                        timestamp_logs[index] = modified_time;
                        println!(
                            "{}The file {:?} has been modified, processing...{}",
                            GREEN, &log.1.name, RESET
                        );

                        let get_lines = read_lines_text_plain(
                            Path::new(&log.1.path)
                        );

                        if let Ok(lines) = get_lines {
                            for element in lines.iter() {
                                let regex = format!("{}", &log.1.regex);

                                let log_serialize = parse_log(
                                  &element,
                                  regex,
                                  &log.1.fields
                                ).expect("Regex no working");

                                println!("{:?}", log_serialize);
                            }
                        }

                    }
                }
            }
        }
    }

}