use aoc_2022::{Result, Helper};

#[derive(Debug)]
pub struct Backpack {
    pub priority: usize,
}

#[derive(Debug)]
pub struct Contents {
    pub first: String,
    pub second: String,
    pub third: String,
}

pub fn find_char(needle: char, haystack: String) -> Option<char> {
    return haystack.chars().find_map(|chr: char| {
        if chr == needle { Some(chr) } else { None }
    });
}

impl Contents {
    pub fn create(items: Vec<String>) -> Result<Self> {
        let content = Self {
            first: String::from(items.get(0).unwrap()),
            second: String::from(items.get(1).unwrap()),
            third: String::from(items.get(2).unwrap()),
        };
        Ok(content)
    }

    pub fn priority(self) -> usize {
        let priority = self.first.chars().into_iter().find_map(|c: char| {
            let second_exists = find_char(c, self.second.clone()).is_some();
            let third_exists = find_char(c, self.third.clone()).is_some();
            if second_exists && third_exists { Some(c) } else { None }
        }).unwrap();
        if priority.is_lowercase() {
            return priority as usize - 'a' as usize + 1;
        }
        return priority as usize - 'A' as usize + 27;
    }
}

pub fn main() -> Result<()> {
    let backpack: Backpack = Helper::new(String::from("day3"))
        .get_input()
        .expect("failed to get input")
        .to_vec('\n')
        .chunks(3)
        .fold(Backpack { priority: 0 }, |mut acc, items| {
            let content = Contents::create(items.to_vec())
                .expect("failed to create contents");
            acc.priority += content.priority();
            return acc;
        });

    dbg!(backpack);
    Ok(())
}
