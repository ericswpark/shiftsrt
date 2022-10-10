use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

use shiftsrt::{RuntimeArguments, TimeCode};

enum LineType {
    Count,
    Timecode,
    Content,
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

    let mut next_line = LineType::Count;
    let mut line_count = 0;

    for line in source_file_reader.lines() {
        line_count += 1;
        let line = line.unwrap();
        match next_line {
            LineType::Count => {
                next_line = LineType::Timecode;
                writeln!(&mut target_file, "{}", line).unwrap();
            }
            LineType::Timecode => {
                next_line = LineType::Content;
                let times: Vec<&str> = line.split(" --> ").collect();
                let mut start_time: TimeCode = TimeCode::parse(times[0]).unwrap_or_else( |e| {
                    println!("There was an error parsing the start timecode on line {line_count}: {e}");
                    process::exit(2);
                });
                let mut end_time: TimeCode = TimeCode::parse(times[1]).unwrap_or_else( |e| {
                    println!("There was an error parsing the end timecode on line {line_count}: {e}");
                    process::exit(2);
                });

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
            LineType::Content => {
                if line.is_empty() {
                    next_line = LineType::Count;
                }
                writeln!(&mut target_file, "{}", line).unwrap();
            }
        }
    }
}
