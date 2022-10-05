

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
