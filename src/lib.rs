use std::fs;
pub mod games;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    pub fn to_vec(self) -> Vec<String> {
        let text = self.text.unwrap();
        let result = text.split("\n").filter_map(|item| {
            if item.is_empty() {
                return None;
            }
            Some(item.to_string())
        }).collect::<Vec<String>>();
        return result;
    }
}

