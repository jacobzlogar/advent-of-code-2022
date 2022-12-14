use aoc_2022::{Result, Helper};

#[derive(Debug)]
struct Elf {
    start: usize,
    end: usize,
}

fn main() -> Result<()> {
    let mut score = 0;
    let collision = Helper::new(String::from("day4"))
        .get_input()
        .expect("failed to get input")
        .to_vec('\n')
        .into_iter()
        .map(|pair| -> Vec<Elf> {
            return pair.split(',')
                .into_iter()
                .map(|elf| -> Elf {
                    let mut it = elf.split('-');
                    let (start, end) = (
                        it.next().unwrap().parse::<usize>().unwrap(),
                        it.next().unwrap().parse::<usize>().unwrap()
                    );
                    Elf {
                        start,
                        end
                    }
                }).collect();
        }).collect::<Vec<Vec<Elf>>>();

    for pair in collision.into_iter() {
        let a = pair[0].start;
        let b = pair[0].end;
        let c = pair[1].start;
        let d = pair[1].end;
        if b >= c && a <= d{
            score += 1;
        }
    }
    dbg!(score);
    Ok(())
}
