use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const TIME_LINE_FORMAT_REGEX: String = "\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}\n";

fn main() {
    // Gather command-line arguments
    let args: Vec<String> = env::args().collect();
    let (path, offset) = handle_args(args);

    println!("Shifting file {} with offset {}.", path, offset);

    // Get source file to read
    let source_file = File::open(path).expect("Failed to open source file.");
    let source_file_reader = BufReader::new(source_file);

    let mut is_time_line = false;

    for line in source_file_reader.lines() {
        // TODO Implement the rest of the logic here
    }
}

fn handle_args(args: Vec<String>) -> (String, i32) {
    // Check if argument count is correct
    const ARG_COUNT: usize = 3;
    if args.len() < ARG_COUNT {
        panic!("Not enough arguments");
    } else if args.len() > ARG_COUNT {
        panic!("Too many arguments");
    }

    // Check if first argument is a valid path and a valid .srt file
    let path = &args[1];
    if !Path::new(&path).exists() {
        panic!("The first argument must be a valid path. The specified path does not exist.");
    }
    if path.len() < 4 || !path.ends_with(".srt") {
        panic!("The file is not a valid .srt file. Hint: make sure the file extension is correct.");
    }

    // Check if second argument is time offset in milliseconds
    let offset: i32 = args[2]
        .trim()
        .parse()
        .expect("Not a valid integer. Input the time offset in milliseconds.");

    (path, offset)
}
