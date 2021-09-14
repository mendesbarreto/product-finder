use dotenv::dotenv;
use std::env;
use std::str::FromStr;

const CHAT_ID_KEY: &str = "CHAT_ID";
const SCHEDULER_TIME_MINUTES_KEY: &str = "SCHEDULER_TIME_MINUTES";

pub fn load() {
    dotenv().ok();
}

pub fn get_application_chat_id() -> i64 {
    let chat_id = env::var_os(CHAT_ID_KEY).unwrap();
    let chat_id_str = chat_id.to_str().unwrap();
    return i64::from_str(chat_id_str).unwrap();
}

pub fn get_application_scheduler_time_minutes() -> u32 {
    let scheduler_time_min = env::var_os(SCHEDULER_TIME_MINUTES_KEY).unwrap();
    let scheduler_time_min_str = scheduler_time_min.to_str().unwrap();
    return u32::from_str(scheduler_time_min_str).unwrap();
}
