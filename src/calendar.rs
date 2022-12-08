use chrono;
use chrono::{Datelike, Timelike};
use serde::*;
use serde_json;

#[derive(Serialize, Debug)]
pub struct CalendarInfo {
    hour: u32,
    minute: u32,
    date: String,
    weekday: chrono::Weekday,
}

pub fn get_calendar_info() -> String {
    let now = chrono::offset::Local::now();
    serde_json::to_string(&CalendarInfo {
        hour: now.hour(),
        minute: now.minute(),
        date: now.date_naive().format("%m.%d").to_string(),
        weekday: now.weekday(),
    })
    .unwrap()
}
