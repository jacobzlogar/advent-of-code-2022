use std::collections::VecDeque;
use std::fs;
pub mod games;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Helper {
    pub day: String,
    pub text: Option<String>,
}

impl Helper {
    pub fn new(day: String) -> Self {
        Self {
            day,
            text: None,
        }
    }

    pub fn get_input(mut self) -> Result<Helper> {
        let result = fs::read_to_string(format!("data/{}.txt", self.day))?;
        self.text = Some(result);
        Ok(self)
    }

    pub fn to_vec(self, splitter: char) -> Vec<String> {
        let text = self.text.unwrap();
        let result = text.split(splitter).filter_map(|item| {
            if item.is_empty() {
                return None;
            }
            Some(item.to_string())
        }).collect::<Vec<String>>();
        return result;
    }
}
#[derive(Debug)]
pub struct Crate {
    items: Vec<String>
}

#[derive(Debug, Copy, Clone)]
pub struct Column {
    //pub row: usize,
    pub column: usize,
    pub value: char,
}

#[derive(Debug, Clone)]
pub struct Row {
    number: usize,
    values: Vec<Column>,
}

#[derive(Debug)]
pub struct Grid {
    pub cols: Vec<VecDeque<Column>>
}

#[derive(Debug)]
pub struct Move {
    pub start: usize,
    pub end: usize,
    pub qty: usize,
}
