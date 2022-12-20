use std::ops::Index;

use aoc_2022::{Result, Helper};

fn main() {
    let data: String = Helper::new(String::from("day6"))
        .get_input()
        .expect("failed to get input")
        .text
        .unwrap();

    let input: Vec<&str> = data
        .split("")
        .skip(1)
        .collect();

    let mut count = 14;
    let mut packet_start = false;
    input.iter().skip(14).fold(input[0..14].to_vec(), |mut acc, x| {
        if is_repeated(acc.clone()) && ! packet_start {
            count += 1;
        } else {
            packet_start = true;
        }
        acc.push(x);
        let output = acc[1..=14].to_vec();
        return output;
    });
    println!("start of packet found at: {}", count);
}

fn is_repeated(data: Vec<&str>) -> bool {
    let search = data.clone();
    search.iter().find_map(|letter| {
        let index = data.iter().position(|x| x.eq(letter)).unwrap();
        let mut new = search.clone();
        new.remove(index);
        new.contains(letter).then(|| true)
    }).is_some()
}
