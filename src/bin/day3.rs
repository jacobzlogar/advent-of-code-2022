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
            let second_exists = self.second.chars().find_map(|chr| {
                if chr == c {
                    return Some(chr);
                }
                None
            });
            let third_exists = self.third.chars().find_map(|chr| {
                if chr == c {
                    return Some(chr);
                }
                None
            });
            if second_exists.is_some() && third_exists.is_some() {
                return Some(c);
            }
            None
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
        .to_vec()
        .chunks(3)
        .fold(Backpack { priority: 0}, |mut acc, items| {
            let content = Contents::create(items.to_vec()).expect("failed to create contents");
            dbg!(&content);
            acc.priority += content.priority();
            return acc;
        });

    dbg!(backpack);
    //dbg!(contents);
    Ok(())
}
