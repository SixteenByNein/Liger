use std::time::SystemTime;

pub fn main() -> String
{
    let printed_time = format!("COMPILATION STARTED AT: {}", to_clock_time(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()));

    return printed_time;



}

fn to_clock_time(secs: u64) -> String
{

    let minutes = secs / 60;
    let hours = minutes / 60;
    let _days = hours / 24;

    return format!("{}:{}:{}", hours % 24, minutes % 60, secs % 60);
}