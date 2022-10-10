use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

use shiftsrt::{RuntimeArguments, TimeCode};

enum LineType {
    COUNT,
    TIMECODE,
    CONTENT,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let runtime_arguments = RuntimeArguments::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Shifting file {} with offset {}.",
        runtime_arguments.source_file_path, runtime_arguments.offset
    );

    // Get source file to read
    let source_file =
        File::open(runtime_arguments.source_file_path).expect("Failed to open source file.");
    let source_file_reader = BufReader::new(source_file);
    let mut target_file =
        File::create(runtime_arguments.target_file_path).expect("Failed to open target file.");

    let mut next_line = LineType::COUNT;

    for line in source_file_reader.lines() {
        let line = line.unwrap();
        match next_line {
            LineType::COUNT => {
                next_line = LineType::TIMECODE;
                writeln!(&mut target_file, "{}", line).unwrap();
            }
            LineType::TIMECODE => {
                next_line = LineType::CONTENT;
                let times: Vec<&str> = line.split(" --> ").collect();
                let mut start_time = TimeCode::parse(times[0]);
                let mut end_time = TimeCode::parse(times[1]);

                start_time.shift(runtime_arguments.offset.into());
                end_time.shift(runtime_arguments.offset.into());

                writeln!(
                    &mut target_file,
                    "{} --> {}",
                    start_time.format_string(),
                    end_time.format_string()
                )
                .unwrap();
            }
            LineType::CONTENT => {
                if line.is_empty() {
                    next_line = LineType::COUNT;
                }
                writeln!(&mut target_file, "{}", line).unwrap();
            }
        }
    }
}
