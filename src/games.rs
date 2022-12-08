use super::{Result, get_input};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strategy {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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

#[derive(Debug)]
pub struct Game {
    pub player_score: usize,
}

impl Game {
    pub fn calculate(&mut self, players: Players) -> usize {
        if players.player == players.opponent {
            return 3;
        }
        return match players.player {
            Shape::Paper => if players.opponent == Shape::Scissors { 0 } else { 6 },
            Shape::Rock => if players.opponent == Shape::Paper { 0 } else { 6 },
            Shape::Scissors => if players.opponent == Shape::Rock { 0 } else { 6 },
        }
    }

    pub fn score(&mut self, players: Players) {
        let round_score = self.calculate(players.clone());
        let total_score = round_score + players.player as usize;
        self.player_score += total_score;
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    pub player: Strategy,
    pub opponent: Shape,
}

#[derive(Debug, Clone)]
pub struct Players {
    pub player: Shape,
    pub opponent: Shape,
}
