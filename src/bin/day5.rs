use std::collections::HashMap;
use aoc_2022::{Result, Helper};

#[derive(Debug)]
struct Crate {
    items: Vec<String>
}

#[derive(Debug, Copy, Clone)]
struct Column {
    row: usize,
    column: usize,
    value: char,
}

#[derive(Debug, Clone)]
struct Row {
    number: usize,
    values: Vec<Column>,
}

#[derive(Debug)]
struct Grid {
    cols: HashMap<usize, Vec<Column>>
}

#[derive(Debug)]
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

    let start = &data[0..8];
    let mut grid = Grid {
        cols: HashMap::new(),
    };
    for (row_number, value) in start.iter().enumerate() {
        let characters: Vec<char> = value.chars().collect();

        for (idx, val) in characters[1..].iter().step_by(4).enumerate() {
            let column_index = idx + 1;
            if val.is_ascii_alphanumeric() {
                let column = Column {
                    row: row_number,
                    column: column_index,
                    value: val.to_owned(),
                };
                if ! grid.cols.contains_key(&column_index) {
                    grid.cols.insert(column_index, vec![column]);
                } else {
                    grid.cols.entry(column_index).and_modify(|entry| {
                        entry.push(column);
                    });
                }
            }
        }
    }

    let moves = &data[9..data.len()];
    for m in moves.iter() {
        let v = m.split_whitespace().collect::<Vec<&str>>();
        let qty = v[1].parse::<usize>().unwrap();
        let from = v[3].parse::<usize>().unwrap();
        let to = v[5].parse::<usize>().unwrap();

        let from_col: Vec<Column> = grid.cols.get(&from).unwrap().to_vec();
        let mut to_col: Vec<Column> = grid.cols.get(&to).unwrap().to_vec();

        let (take, remainder) = &from_col.split_at(qty);

        for t in remainder.iter().rev() {
            to_col.push(Column {
                column: to,
                row: t.row,
                value: t.value
            });
        }
        grid.cols.insert(from, take.to_vec());
        grid.cols.insert(to, to_col);
    }
    for col in grid.cols {
        dbg!(col.1[col.1.len() - 1]);
    }

    Ok(())
}
