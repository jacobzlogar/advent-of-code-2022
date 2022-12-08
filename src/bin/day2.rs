use aoc_2022::{Result, Game, Shape, Round, Strategy, get_input, Players};

pub fn main() -> Result<()> {
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
            Some(Players {
                opponent: round.opponent,
                player: Shape::try_from(round).unwrap(),
            })
        }
    );

    rounds.for_each(|round| {
        game.score(round);
    });
    println!("score: {}", game.player_score);
    Ok(())
}
