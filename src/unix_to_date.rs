use std::string::String;

pub fn new(sec: u64) -> String {
    let mut total_seconds = sec;
    let days = total_seconds / 86400;
    total_seconds %= 86400;
    let hours = total_seconds / 3600;
    total_seconds %= 3600;
    let minutes = total_seconds / 60;
    total_seconds %= 60;
    let seconds = total_seconds;

    return format!(
        "{} days {} hours {} minutes {} seconds",
        days, hours, minutes, seconds
    );
}
