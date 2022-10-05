use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

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
        // TODO Implement the rest of the logic here
    }
}

fn handle_args(args: Vec<String>) -> (String, String, i32) {
    // Check if argument count is correct
    const ARG_COUNT: usize = 3;
    if args.len() < ARG_COUNT {
        panic!("Not enough arguments");
    } else if args.len() > ARG_COUNT {
        panic!("Too many arguments");
    }

    // Check if first argument is a valid path and a valid .srt file
    let source_path = &args[1];
    if !Path::new(&source_path).exists() {
        panic!("The first argument must be a valid path. The specified path does not exist.");
    }
    if source_path.len() < 4 || !source_path.ends_with(".srt") {
        panic!("The file is not a valid .srt file. Hint: make sure the file extension is correct.");
    }

    // Check if second argument is time offset in milliseconds
    let offset: i32 = args[2]
        .trim()
        .parse()
        .expect("Not a valid integer. Input the time offset in milliseconds.");

    (source_path.clone(), target_path, offset)
}
