use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn day_1() -> Result<String> {
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
    Ok(sum.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strategy {
    Lose = 0,
    Tie = 3,
    Win = 6,
}

impl From<&str> for Strategy {
    fn from(value: &str) -> Strategy {
        return match value {
            "X" => Strategy::Lose,
            "Y" => Strategy::Tie,
            _ => Strategy::Win,
        }
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Shape {
        return match value {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            _ => Shape::Scissors,
        };
    }
}

impl TryFrom<Round> for Shape {
    type Error = Box<dyn std::error::Error>;

    fn try_from(values: Round) -> Result<Shape> {
        return match values.player {
            Strategy::Tie => Ok(values.opponent),
            Strategy::Win => {
                return match values.opponent {
                    Shape::Paper => Ok(Shape::Scissors),
                    Shape::Rock => Ok(Shape::Paper),
                    Shape::Scissors => Ok(Shape::Rock),
                }
            },
            Strategy::Lose => {
                return match values.opponent {
                    Shape::Paper => Ok(Shape::Rock),
                    Shape::Rock => Ok(Shape::Scissors),
                    Shape::Scissors => Ok(Shape::Paper),
                }
            }
        };
    }
}

fn calculate(hand: Shape, opponent: Shape) -> usize {
    if hand == opponent {
        return 3;
    }
    return match hand {
        Shape::Paper => if opponent == Shape::Scissors { 0 } else { 6 },
        Shape::Rock => if opponent == Shape::Paper { 0 } else { 6 },
        Shape::Scissors => if opponent == Shape::Rock { 0 } else { 6 },
    }
}

#[derive(Debug)]
struct Game {
    player_score: usize,
}

impl Game {
    fn score(&mut self, players: Play) {
        let round_score = calculate(players.player, players.opponent);
        let total_score = round_score + players.player as usize;
        self.player_score += total_score;
    }
}

#[derive(Debug, Clone)]
struct Round {
    player: Strategy,
    opponent: Shape,
}

#[derive(Debug, Clone)]
struct Play {
    player: Shape,
    opponent: Shape,
}


pub fn day_2() -> Result<usize> {
    let mut game = Game { player_score: 0 };
    let text = get_input("day2")?;
    let rounds = text.split("\n").filter_map(|round|
        if round.is_empty() { None }
        else {
            let mut round = round.split_whitespace().into_iter();
            let round = Round {
                opponent: Shape::from(round.next().unwrap()),
                player: Strategy::from(round.next().unwrap()),
            };
            let play = Play {
                opponent: round.opponent,
                player: Shape::try_from(round).unwrap(),
            };
            Some(play)
        }
    );

    rounds.for_each(|round| {
        game.score(round);
    });
    Ok(game.player_score)
}

fn get_input(day: &str) -> Result<String> {
    Ok(fs::read_to_string(format!("data/{}.txt", day))?)
}
