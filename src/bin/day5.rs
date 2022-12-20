use std::collections::VecDeque;
use aoc_2022::{Helper, Grid, Column, Move};

pub fn parse_move(index: usize, moves: Vec<String>) -> usize {
    moves[index].parse::<usize>().unwrap()
}

pub fn parse_moves(data: Vec<String>) -> Vec<Move> {
    data[9..data.len()]
        .into_iter()
        .map(|mov| {
            let text = mov
                .split_whitespace()
                .map(|i| i.to_string())
                .collect::<Vec<String>>();

            return Move {
                qty: parse_move(1,  text.clone()),
                start: parse_move(3, text.clone()),
                end: parse_move(5, text.clone()),
            }
        })
        .collect()
}
pub fn create_grid() -> (Vec<String>, Grid) {
    let data: Vec<String> = Helper::new(String::from("day5"))
        .get_input()
        .expect("failed to get input")
        .to_vec('\n');

    let mut grid = Grid {
        cols: vec![VecDeque::new(); 9]
    };
    // first 8 lines represent the initial crate setup
    for (_row_number, row_val) in data[0..8]
        .into_iter()
        .enumerate() {
            let cols: Vec<Column> = row_val.chars()
                .into_iter()
                .skip(1)
                .step_by(4)
                .enumerate()
                .map(|(idx, val)| {
                    Column {
                        column: idx,
                        value: val,
                    }
                }).collect();
            cols.into_iter().rev().for_each(|c| {
                if ! c.value.to_string().trim().is_empty() {
                    grid.cols[c.column].push_front(c);
                }
        });
    }
    return (data, grid);
}

fn main() {
    let (data, mut grid) = create_grid();
    let moves = parse_moves(data);
    for mov in moves {
        let mut grid_cols = grid.cols.clone();
        let start_row = &mut grid_cols[mov.start - 1];
        let insert = start_row.iter().rev().take(mov.qty).collect::<VecDeque<&Column>>();
        if insert.len().eq(&1) {
            let col = Column {
                column: mov.end - 1,
                value: insert[0].value
            };
            grid.cols[mov.end - 1].push_back(col);
            grid.cols[mov.start - 1].pop_back();
        } else {
            for take in insert.into_iter().rev() {
                let col = Column {
                    column: mov.end - 1,
                    value: take.value
                };
                grid.cols[mov.end - 1].push_back(col);
                grid.cols[mov.start - 1].pop_back();
            }
        }
    }

    let mut result = String::new();
    for col in grid.cols {
        dbg!(&col);
        result.push(col[col.len() - 1].value);
    }
    dbg!(result);
}
