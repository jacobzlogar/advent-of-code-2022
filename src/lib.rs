use std::fs;
pub mod games;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_input(day: &str) -> Result<String> {
    Ok(fs::read_to_string(format!("data/{}.txt", day))?)
}
