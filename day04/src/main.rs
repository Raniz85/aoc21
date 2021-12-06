use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    worst: bool,
}

#[derive(Eq, Debug, PartialEq)]
struct BingoBoard([[u8; 5]; 5]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BingoResult(usize, u32);

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let mut iter = input.split('\n');
    let bingo_numbers = iter
        .next()
        .map(|line| {
            line.split(',')
                .map(|n| Ok(u8::from_str(n)?))
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or_else(|| Err(anyhow!("No input")))?;
    let mut boards = Vec::new();
    loop {
        match BingoBoard::new(&mut iter)? {
            Some(board) => boards.push(board),
            None => break,
        }
    }
    let results = boards
        .iter()
        .filter_map(|board| board.bingo(bingo_numbers.as_slice()));
    let best = if opts.worst {
        results.max_by_key(|b| b.0)
    } else {
        results.min_by_key(|b| b.0)
    }
    .ok_or_else(|| anyhow!("No boards got bingo"))?;
    println!("Bingo in {} steps with score of {}", best.0, best.1);
    Ok(())
}

impl BingoBoard {
    pub fn new(input: &mut dyn Iterator<Item = &str>) -> Result<Option<BingoBoard>> {
        let mut board = BingoBoard([[0u8; 5]; 5]);
        let mut row = 0;
        loop {
            match input.next() {
                Some("") => continue,
                Some(line) => {
                    let numbers = line
                        .split(' ')
                        .filter(|n| n != &"")
                        .map(|n| Ok(u8::from_str(n)?))
                        .collect::<Result<Vec<_>>>()?;
                    match numbers.len() {
                        5 => numbers
                            .iter()
                            .enumerate()
                            .for_each(|(col, n)| board.0[row][col] = *n),
                        _ => bail!("Invalid line {}", line),
                    };
                    if row == 4 {
                        return Ok(Some(board));
                    }
                }
                None => return Ok(None),
            }
            row += 1;
        }
    }

    pub fn bingo(&self, numbers: &[u8]) -> Option<BingoResult> {
        let mut numbers_hit: Vec<u32> = Vec::new();
        let mut rows = [0u8; 5];
        let mut cols = [0u8; 5];
        for (step, number) in numbers.iter().enumerate() {
            for row in 0..5 {
                for col in 0..5 {
                    if self.0[row][col] == *number {
                        numbers_hit.push(*number as u32);
                        rows[row] += 1;
                        cols[col] += 1;
                        if rows[row] == 5 || cols[col] == 5 {
                            let board_sum = self.sum();
                            let hit_sum: u32 = numbers_hit.iter().sum();
                            let unhit_sum = board_sum - hit_sum;
                            return Some(BingoResult(step, *number as u32 * unhit_sum));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn sum(&self) -> u32 {
        self.0
            .iter()
            .flat_map(|r| r.iter())
            .map(|n| *n as u32)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::{BingoBoard, BingoResult};

    #[test]
    fn test_new_board() {
        let input = include_str!("test_input");
        let expected = [
            BingoBoard([
                [22u8, 13u8, 17u8, 11u8, 0u8],
                [8u8, 2u8, 23u8, 4u8, 24u8],
                [21u8, 9u8, 14u8, 16u8, 7u8],
                [6u8, 10u8, 3u8, 18u8, 5u8],
                [1u8, 12u8, 20u8, 15u8, 19u8],
            ]),
            BingoBoard([
                [3u8, 15u8, 0u8, 2u8, 22u8],
                [9u8, 18u8, 13u8, 17u8, 5u8],
                [19u8, 8u8, 7u8, 25u8, 23u8],
                [20u8, 11u8, 10u8, 24u8, 4u8],
                [14u8, 21u8, 16u8, 12u8, 6u8],
            ]),
            BingoBoard([
                [14u8, 21u8, 17u8, 24u8, 4u8],
                [10u8, 16u8, 15u8, 9u8, 19u8],
                [18u8, 8u8, 23u8, 26u8, 20u8],
                [22u8, 11u8, 13u8, 6u8, 5u8],
                [2u8, 0u8, 12u8, 3u8, 7u8],
            ]),
        ];

        let mut lines = input.split('\n');
        let boards = [
            BingoBoard::new(&mut lines),
            BingoBoard::new(&mut lines),
            BingoBoard::new(&mut lines),
        ];

        expected
            .iter()
            .zip(boards.into_iter())
            .for_each(|(expected, board)| {
                assert!(board.is_ok());
                let board = board.unwrap();
                assert!(board.is_some());
                assert_eq!(expected, &board.unwrap());
            });
    }

    #[test]
    fn test_bingo() {
        let boards = [
            BingoBoard([
                [22u8, 13u8, 17u8, 11u8, 0u8],
                [8u8, 2u8, 23u8, 4u8, 24u8],
                [21u8, 9u8, 14u8, 16u8, 7u8],
                [6u8, 10u8, 3u8, 18u8, 5u8],
                [1u8, 12u8, 20u8, 15u8, 19u8],
            ]),
            BingoBoard([
                [3u8, 15u8, 0u8, 2u8, 22u8],
                [9u8, 18u8, 13u8, 17u8, 5u8],
                [19u8, 8u8, 7u8, 25u8, 23u8],
                [20u8, 11u8, 10u8, 24u8, 4u8],
                [14u8, 21u8, 16u8, 12u8, 6u8],
            ]),
            BingoBoard([
                [14u8, 21u8, 17u8, 24u8, 4u8],
                [10u8, 16u8, 15u8, 9u8, 19u8],
                [18u8, 8u8, 23u8, 26u8, 20u8],
                [22u8, 11u8, 13u8, 6u8, 5u8],
                [2u8, 0u8, 12u8, 3u8, 7u8],
            ]),
        ];
        let numbers = [
            7u8, 4u8, 9u8, 5u8, 11u8, 17u8, 23u8, 2u8, 0u8, 14u8, 21u8, 24u8, 10u8, 16u8, 13u8,
            6u8, 15u8, 25u8, 12u8, 22u8, 18u8, 20u8, 8u8, 19u8, 3u8, 26u8, 1u8,
        ];
        let results = boards
            .iter()
            .filter_map(|board| board.bingo(&numbers))
            .collect::<Vec<_>>();
        assert_eq!(
            vec![
                BingoResult(13, 2192),
                BingoResult(14, 1924),
                BingoResult(11, 4512)
            ],
            results
        );
    }
}
