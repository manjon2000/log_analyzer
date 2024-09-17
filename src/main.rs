mod pattern;
mod parse;
mod filter;
mod file;
mod date;

use std::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use chrono::{NaiveDate, NaiveTime};
use crate::file::read_lines_text_plain;
use crate::parse::parse_log;
use crate::pattern::*;
use clap::{Parser, Subcommand};
use clap::builder::Str;
use crate::filter::filter_by_key_value;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    analyzer: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    search {
        #[arg(short)]
        /// Search by key formating is key,value. Example ip,127.0.0.1
        key: Option<String>,
        #[arg(short)]
        /// Search by range date is key,start%send. Example date,09/May/2024%s10/May/2024
        range_date: Option<String>,

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_config = File::open("./config.log.json")
        .expect("Error open file");
    let reader = BufReader::new(file_config);
    let config: LogConfig = serde_json::from_reader(reader)?;

    let config_cloned = Box::leak(Box::new(config.logs));
    let all_logs: &'static mut Vec<HashMap<String, String>> = Box::leak(Box::new(Vec::new()));

    for log in config_cloned.iter() {
        match log.1.type_log {
            TypeLogPattern::TextPlain => {
                let content = read_lines_text_plain(
                    Path::new(&log.1.path)
                );
                if let Ok(lines) = content {
                    for element in lines.iter() {
                        let format_log = parse_log(
                            element,
                            log.1.regex.to_string(),
                            &log.1.fields,
                        );

                        if let Ok(logs) = format_log {
                            all_logs.push(logs);
                        }
                    }

                }
            },
            _ => {
               panic!("Not exist method 😔!!!")
            }
        }
    }

    let args = Args::parse();
    match args.analyzer {
        Commands::search{key, range_date} => {
            if let Some(search) = key {
                let get_key_value = search.split(",")
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>();
                assert_eq!(get_key_value.len(), 2,
                           "\n {RED}Format error. The format has to be key,value.{RESET} \
                           \n {GREEN}Example ip,127.0.0.1 {RESET}");
                let field_key = &get_key_value[0];
                let value = &get_key_value[1];

                let results = filter_by_key_value(
                    all_logs,
                    field_key,
                    value
                );

                println!("{:?}", results);
            }
            if let Some(range) = range_date {
                let separate_string = range.split(",")
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>();
                let separate_dates = &separate_string[1].split("%s")
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();
                
                println!("{BLUE}Date Start: {:?} -> Date End: {:?} {RESET}", 
                    separate_dates[0],
                    separate_dates[1]
                );

                let items_filter = get_logs_within_dates(
                    all_logs,
                    &separate_string[0],
                    &separate_dates[0],
                    &separate_dates[1]
                );

                println!("{:?}", items_filter);
            }
        }
    }
    Ok(())
}

fn get_logs_within_dates(
    logs: &[HashMap<String, String>],
    key: &String,
    start: &str,
    end: &str
) -> Vec<HashMap<String, String>> {
    const DATE_FORMAT: &str = "%d/%b/%Y";
    let date_start = NaiveDate::parse_from_str(start, DATE_FORMAT).unwrap();
    let date_end = NaiveDate::parse_from_str(end, DATE_FORMAT).unwrap();
    let mut logs_range: Vec<HashMap<String, String>> = Vec::new();

    for eL in  logs.iter() {
        if let Some(value) = eL.get(key) {
            let current_date = value.split(":")
                .map(|e| e.to_string())
                .collect::<Vec<String>>();

            let convert_to_format_unix = NaiveDate::parse_from_str(
                current_date[0].trim(),
                DATE_FORMAT
            );

            if let Ok(current_date) = convert_to_format_unix {

                if current_date >= date_start && current_date <= date_end {
                    println!("{:?}", eL);
                    logs_range.push(eL.clone());
                }
            }

            if let Err(err) = convert_to_format_unix {
                println!("{:?}", err.to_string());
            }
        }
    }

    logs_range
}