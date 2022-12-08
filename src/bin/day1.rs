use aoc_2022::{Result, get_input};

pub fn main() -> Result<()> {
    // get the input, split it, maybe this should be in lib.rs
    let text = get_input("day1")?;
    let split: Vec<&str> = text.split("\n\n").collect();
    let data: Vec<Vec<u64>> = split
        .iter()
        .map(|value| {
            let items: Vec<u64> = value
                .split_whitespace()
                .map(|item| {
                    return item.parse::<u64>().unwrap();
                })
                .collect();
            return items;
        })
        .collect();
    // map the sum of calories
    let mut sums: Vec<u64> = data
        .into_iter()
        .map(|current: Vec<u64>| current.clone().into_iter().sum::<u64>())
        .collect();

    // ya we sortin
    sums.sort();

    // take the beeg 3 and sum them
    let sum = &sums[sums.len() - 3..].into_iter().sum::<u64>();
    println!("calories: {}", sum.to_string());
    Ok(())
}
