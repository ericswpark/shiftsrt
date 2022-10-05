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
        if args.len() < ARG_COUNT {
            return Err("Not enough arguments");
        } else if args.len() > ARG_COUNT {
            return Err("Too many arguments");
        }

        // Check if first argument is a valid path and a valid .srt file
        let source_file_path = &args[1];
        if !Path::new(&source_file_path).exists() {
            return Err("The first argument must be a valid path. The specified path does not exist.");
        }
        if source_file_path.len() < 4 || !source_file_path.ends_with(".srt") {
            return Err("The file is not a valid .srt file. Hint: make sure the file extension is correct.");
        }

        // Check if target file already exists
        let target_file_path = source_file_path
            .get(..source_file_path.len()-4)
            .unwrap()
            .to_owned() + "-shift.srt";
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
            offset
        })
    }
}

impl TimeCode {
    pub fn new(hour: u8, minute: u8, second: u8, millisecond: u16) -> TimeCode {
        TimeCode { hour, minute, second, millisecond }
    }

    pub fn format_string(&self) -> String {
        format!("{:02}:{:02}:{:02},{:03}",
            self.hour,
            self.minute,
            self.second,
            self.millisecond
        )
    }
}

pub fn shift(time: String, offset: i32) -> String {
    let parts: Vec<&str> = time.split(",").collect();
    let millisecond: u16 = parts[1].trim().parse().unwrap();

    let parts: Vec<&str> = parts[0].split(":").collect();
    let hour: u8 = parts[0].trim().parse().unwrap();
    let minute: u8 = parts[1].trim().parse().unwrap();
    let second: u8 = parts[2].trim().parse().unwrap();

    println!("Hour: {}, Minute: {}, Second: {}", hour, minute, second);
    let second: i64 = (i64::from(second) + i64::from(minute) * 60 + i64::from(hour) * 60 * 60).into();
    let millisecond: i64 = (i64::from(millisecond) + second * 1000).into();

    let mut millisecond = millisecond + i64::from(offset);

    println!("Millisecond: {}", millisecond);
    let hour: u8 = (millisecond / (1000 * 60 * 60)).try_into().unwrap();
    millisecond %= 1000 * 60 * 60;
    let minute: u8 = (millisecond / (1000 * 60)).try_into().unwrap();
    millisecond %= 1000 * 60;
    let second: u8 = (millisecond / 1000).try_into().unwrap();
    millisecond %= 1000;

    format!("{}:{}:{},{}", hour, minute, second, millisecond)
}
