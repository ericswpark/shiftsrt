use std::num::TryFromIntError;
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

impl RuntimeArguments {
    pub fn build(args: [String; 3]) -> Result<RuntimeArguments, &'static str> {
        let [_, source_file_path, offset] = args;

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
        let offset: i32 = offset.trim().parse().or(Err(
            "Not a valid integer. Input the time offset in milliseconds.",
        ))?;

        Ok(RuntimeArguments {
            source_file_path,
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

    pub fn parse(timecode_string: &str) -> Result<TimeCode, &'static str> {
        let parts: Vec<&str> = timecode_string.split(',').collect();
        if parts.len() < 2 {
            return Err("Not enough parts to timecode string.");
        }
        let millisecond: u16 = parts[1]
            .trim()
            .parse()
            .or(Err("Millisecond cannot be parsed."))?;
        let parts: Vec<&str> = parts[0].split(':').collect();
        let hour: u8 = parts[0].trim().parse().or(Err("Hour cannot be parsed."))?;
        let minute: u8 = parts[1]
            .trim()
            .parse()
            .or(Err("Minute cannot be parsed."))?;
        let second: u8 = parts[2]
            .trim()
            .parse()
            .or(Err("Second cannot be parsed."))?;

        // Bounds check
        if hour > 99 {
            return Err("Hour cannot be more than 99 hours.");
        }
        if minute > 59 {
            return Err("Minute cannot be more than 59 minutes.");
        }
        if second > 59 {
            return Err("Second cannot be more than 59 seconds.");
        }
        if millisecond > 999 {
            return Err("Millisecond cannot be more than 999 milliseconds.");
        }

        Ok(TimeCode {
            hour,
            minute,
            second,
            millisecond,
        })
    }

    fn get_millisecond_in_total(&self) -> u64 {
        self.hour as u64 * (60 * 60 * 1000)
            + self.minute as u64 * (60 * 1000)
            + self.second as u64 * 1000
            + self.millisecond as u64
    }

    fn millisecond_to_timecode(millisecond: u64) -> Result<TimeCode, TryFromIntError> {
        let hour: u8 = (millisecond / (60 * 60 * 1000)).try_into()?;
        let millisecond = millisecond % (60 * 60 * 1000);
        let minute: u8 = (millisecond / (60 * 1000)).try_into()?;
        let millisecond = millisecond % (60 * 1000);
        let second: u8 = (millisecond / 1000).try_into()?;
        let millisecond: u16 = (millisecond % 1000).try_into()?;

        Ok(TimeCode {
            hour,
            minute,
            second,
            millisecond,
        })
    }

    pub fn shift(&mut self, offset: i64) {
        let new_millisecond = match (self.get_millisecond_in_total() as i64).checked_add(offset) {
            Some(x) => x,
            None => panic!("Cannot shift timecode by offset {offset}, value overflows."),
        };
        *self = TimeCode::millisecond_to_timecode(new_millisecond.try_into().unwrap()).unwrap();
    }

    pub fn format_string(&self) -> String {
        format!(
            "{:02}:{:02}:{:02},{:03}",
            self.hour, self.minute, self.second, self.millisecond
        )
    }
}
