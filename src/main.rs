mod pattern;
mod parse;
mod filter;
mod file;


use std::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use crate::file::read_lines_text_plain;
use crate::parse::parse_log;
use crate::pattern::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_config = File::open("./config.log.json")
        .expect("Error open file");
    let reader = BufReader::new(file_config);
    let config: LogConfig = serde_json::from_reader(reader)?;


    let config_cloned = Box::leak(Box::new(config.logs));

    for log in config_cloned.iter() {
        let content = read_lines_text_plain(
            Path::new(&log.1.path)
        );

        if let Ok(lines) = content {
            println!("{:?}", lines.len());
            for element in lines.iter() {
                let format_regex = format!("{}", &log.1.regex);
                let format_log = parse_log(
                    element,
                    format_regex,
                    &log.1.fields,
                );

                if let Ok(logs) = format_log {
                    println!("{:?} \n\n", logs);
                }
            }
        }
    }

    Ok(())
}
