use aoc_2022::{Result, Helper};

#[derive(Debug)]
struct Crate {
    items: Vec<String>
}

struct Move {
    start: usize,
    end: usize,
    crates: Vec<Crate>
}

fn main() -> Result<()> {
    let data: Vec<String> = Helper::new(String::from("day5"))
        .get_input()
        .expect("failed to get input")
        .to_vec('\n');

    let values = &data[0..=8];
    let moves = &data[10..data.len()];
    let first_chars = &values[0].chars().collect::<Vec<char>>();
    dbg!(first_chars);

    Ok(())
}
