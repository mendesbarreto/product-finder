use dotenv::dotenv;
use std::str::FromStr;

pub fn load() {
    dotenv().ok();
}

pub fn get_application_chat_id() -> i64 {
    let value = (dotenv_codegen::dotenv!("CHAT_ID")).to_string();
    return i64::from_str(value.as_str()).unwrap();
}
