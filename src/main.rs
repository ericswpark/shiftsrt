use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use shiftsrt::*;

const TIME_LINE_FORMAT_REGEX: &str = r"\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}";

fn main() {
    // Gather command-line arguments
    let (source_path, target_path, offset) = handle_args(env::args().collect());

    println!("Shifting file {} with offset {}.", source_path, offset);

    // Get source file to read
    let source_file = File::open(source_path).expect("Failed to open source file.");
    let source_file_reader = BufReader::new(source_file);
    let mut target_file = File::create(target_path).expect("Failed to open target file.");

    let mut is_time_line = false;
    let regex_matcher = Regex::new(TIME_LINE_FORMAT_REGEX).unwrap();

    for line in source_file_reader.lines() {
        let line = line.unwrap();
        if !regex_matcher.is_match(&line) {
            // Not a time line, write directly to file
            writeln!(&mut target_file, "{}", line).unwrap();
        } else {
            let times: Vec<&str> = line.split(" --> ").collect();
            let start_time = times[0];
            let end_time = times[1];

            let start_time = shift(start_time.to_string(), offset);
            let end_time = shift(end_time.to_string(), offset);
            writeln!(&mut target_file, "{} --> {}", start_time, end_time).unwrap();
        }
    }
}
