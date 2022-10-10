use std::cmp::Ordering;
use std::path::Path;

pub struct RuntimeArguments {
    pub source_file_path: String,
    pub target_file_path: String,
    pub offset: i32,
}

pub struct TimeCode {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

const ARG_COUNT: usize = 3;

impl RuntimeArguments {
    pub fn build(args: &[String]) -> Result<RuntimeArguments, &'static str> {
        // Check if argument count is correct
        match args.len().cmp(&ARG_COUNT) {
            Ordering::Less => return Err("Not enough arguments"),
            Ordering::Greater => return Err("Too many arguments"),
            Ordering::Equal => (),
        }

        // Check if first argument is a valid path and a valid .srt file
        let source_file_path = &args[1];
        if !Path::new(&source_file_path).exists() {
            return Err(
                "The first argument must be a valid path. The specified path does not exist.",
            );
        }
        if source_file_path.len() < 4 || !source_file_path.ends_with(".srt") {
            return Err(
                "The file is not a valid .srt file. Hint: make sure the file extension is correct.",
            );
        }

        // Check if target file already exists
        let target_file_path = source_file_path
            .get(..source_file_path.len() - 4)
            .unwrap()
            .to_owned()
            + "-shift.srt";
        if Path::new(&target_file_path).exists() {
            return Err("The target file exists. To prevent accidentally overwriting the file, shiftsrt will now stop.");
        }

        // Check if second argument is time offset in milliseconds
        let offset: i32 = args[2]
            .trim()
            .parse()
            .expect("Not a valid integer. Input the time offset in milliseconds.");

        Ok(RuntimeArguments {
            source_file_path: source_file_path.clone(),
            target_file_path,
            offset,
        })
    }
}

impl TimeCode {
    pub fn new(hour: u8, minute: u8, second: u8, millisecond: u16) -> TimeCode {
        TimeCode {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    pub fn parse(timecode_string: &str) -> TimeCode {
        let parts: Vec<&str> = timecode_string.split(',').collect();
        let millisecond: u16 = parts[1].trim().parse().unwrap();
        let parts: Vec<&str> = parts[0].split(':').collect();
        let hour: u8 = parts[0].trim().parse().unwrap();
        let minute: u8 = parts[1].trim().parse().unwrap();
        let second: u8 = parts[2].trim().parse().unwrap();

        TimeCode {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    fn get_millisecond_in_total(&self) -> u64 {
        self.hour as u64 * (60 * 60 * 1000)
            + self.minute as u64 * (60 * 1000)
            + self.second as u64 * 1000
            + self.millisecond as u64
    }

    fn millisecond_to_timecode(millisecond: u64) -> TimeCode {
        let hour: u8 = (millisecond / (60 * 60 * 1000)).try_into().unwrap();
        let millisecond = millisecond % (60 * 60 * 1000);
        let minute: u8 = (millisecond / (60 * 1000)).try_into().unwrap();
        let millisecond = millisecond % (60 * 1000);
        let second: u8 = (millisecond / 1000).try_into().unwrap();
        let millisecond: u16 = (millisecond % 1000).try_into().unwrap();

        TimeCode {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    pub fn shift(&mut self, offset: i64) {
        let new_millisecond = self.get_millisecond_in_total() as i64 + offset;
        *self = TimeCode::millisecond_to_timecode(new_millisecond.try_into().unwrap());
    }

    pub fn format_string(&self) -> String {
        format!(
            "{:02}:{:02}:{:02},{:03}",
            self.hour, self.minute, self.second, self.millisecond
        )
    }
}
