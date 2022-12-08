use chrono;
use chrono::{Datelike, Timelike};
use serde::*;
use serde_json;

#[derive(Serialize, Debug)]
pub struct CalendarInfo {
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
    weekday: chrono::Weekday,
}

pub fn get_calendar_info() -> String {
    let now = chrono::offset::Local::now();
    serde_json::to_string(&CalendarInfo {
        year: format!("{:04}", now.year()),
        month: format!("{:02}", now.month()),
        day: format!("{:02}", now.day()),
        hour: format!("{:02}", now.hour()),
        minute: format!("{:02}", now.minute()),
        weekday: now.weekday(),
    })
    .unwrap()
}
