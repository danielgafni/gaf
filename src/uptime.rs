use psutil::host::uptime;


pub fn get_uptime_hours() -> String {
    let uptime_seconds = uptime().unwrap().as_secs();
    format!("{}", uptime_seconds / 3600)
}

pub fn get_uptime_minutes() -> String {
    format!("{:2}", uptime().unwrap().as_secs() / 60 % 60)
}
