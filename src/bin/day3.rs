use aoc_2022::{Result, Helper};

#[derive(Debug)]
pub struct Backpack {
    pub priority: usize,
}

#[derive(Debug)]
pub struct Contents {
    pub first: String,
    pub second: String,
}

impl Contents {
    pub fn create(items: String) -> Self {
        let (first, second) = items.split_at(items.len() / 2);
        Self {
            first: String::from(first),
            second: String::from(second),
        }
    }

    pub fn priority(self) -> usize {
        let char = self.first.chars().into_iter().find_map(|c: char| {
            self.second.chars().find(|char| {
                char == &c
            })
        }).unwrap();
        if char.is_lowercase() {
            return char as usize - 'a' as usize + 1;
        }
        return char as usize - 'A' as usize + 27;
    }
}

pub fn main() -> Result<()> {
    let backpack: Backpack = Helper::new(String::from("day3"))
        .get_input()
        .expect("failed to get input")
        .to_vec()
        .iter()
        .fold(Backpack { priority: 0}, |mut acc, items| {
            let content = Contents::create(String::from(items));
            acc.priority += content.priority();
            return acc;
        });

    dbg!(backpack);
    //dbg!(contents);
    Ok(())
}
