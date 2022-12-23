use aoc_2022::Helper;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use aoc_2022::Helper;
    use crate::solve_part_1;
    #[test]
    fn test_solve_part_1() {
        let txt = String::from("1111\n1211\n1111");
        let puzzle_input = Helper {
            day: String::from("day8"),
            text: Some(txt),
        }.to_vec('\n');

        assert_eq!(solve_part_1(puzzle_input), 11);
    }
    fn test_solve_part_2() {
    }
}

/// TODO:build the grid of trees for part1/2
struct GridItem {
    row: Vec<String>,
    column: Vec<String>,
}

fn visible(trees: String, from: usize, to: usize, check: char) -> bool {
    if from.eq(&to) {
        return true;
    }
    trees.chars()
       .into_iter()
       .skip(from)
       .take(to)
       .filter(|l| l.ge(&check))
       .collect::<Vec<char>>()
       .len()
       .eq(&0)
}

/// TODO:build the grid of trees for part1/2
fn build_grid(puzzle_input: Vec<String>) -> Vec<HashMap<char, GridItem>> {
    puzzle_input.into_iter().enumerate().map(|(row_idx, line)| {
        return line.chars()
            .clone()
            .into_iter()
            .enumerate()
            .map(|(col_idx, val)| {
                let res = HashMap::new();
                let row = puzzle_input[row_idx]
                    .clone()
                    .split("")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let column = puzzle_input
                    .clone()
                    .into_iter()
                    .map(|c| c.chars().collect::<Vec<char>>()[col_idx].to_string())
                    .collect::<Vec<String>>();
                let grid_item = GridItem {
                    row,
                    column
                };
                res.insert(val, GridItem {
                    row,
                    column
                });
                return res;
            }).unzip();
    }).collect()
}

fn solve_part_1(puzzle_input: Vec<String>) -> usize {
    puzzle_input.iter().enumerate().fold(0, |mut acc, (row_idx, line)| {
        line.chars()
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(col_idx, val)| {
                let row = puzzle_input[row_idx].clone();
                let column = puzzle_input
                    .clone()
                    .into_iter()
                    .map(|c| c.chars().collect::<Vec<char>>()[col_idx].to_string())
                    .collect::<Vec<String>>()
                    .join("");
                let left = visible(row.clone(), 0, col_idx, val);
                let right = visible(row.clone(), col_idx + 1, row.clone().len(), val);
                let above = visible(column.clone(), 0, row_idx, val);
                let below = visible(column.clone(), row_idx + 1, row.clone().len(), val);
                let visible = vec![left, right, above, below]
                    .into_iter()
                    .filter(|f| *f)
                    .collect::<Vec<bool>>();
                if ! visible.is_empty() {
                    acc += 1;
                }
            });
        return acc
    })
}

fn main() {
    let puzzle_input = Helper::new(String::from("day8"))
        .get_input()
        .expect("failed to get day 8 input, is it there?")
        .to_vec('\n');

    let part_1_solution = solve_part_1(puzzle_input.clone());
    println!("part 1 solution: {}", part_1_solution);
}
