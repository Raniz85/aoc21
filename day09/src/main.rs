use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    basins: bool,
}

struct Map<const N: usize, const M: usize>([[u8; M]; N]);

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let map: Map<100, 100> = Map::parse(input.split('\n'))?;
    let score = if opts.basins {
        map.find_basins()
            .into_iter()
            .map(|basin| basin.len())
            .sorted()
            .rev()
            .take(3)
            .product()
    } else {
        let lowpoints = map.find_lowpoints();
        lowpoints.iter().map(|n| *n as usize).sum::<usize>() + lowpoints.len()
    };
    println!("{}", score);
    Ok(())
}

impl<const N: usize, const M: usize> Map<N, M> {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Map<N, M>> {
        Ok(Map(lines
            .map(|line| -> Result<[u8; M]> {
                line.chars()
                    .map(|c| match c {
                        '0'..='9' => Ok(c.to_digit(10).unwrap() as u8),
                        _ => Err(anyhow!("Invalid integer {}", c)),
                    })
                    .collect::<Result<Vec<u8>>>()?
                    .try_into()
                    .map_err(|v: Vec<u8>| {
                        anyhow!("Expected {} columns but was only {}", M, v.len())
                    })
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|v: Vec<[u8; M]>| {
                anyhow!("Expected {} rows but was only {}", N, v.len())
            })?))
    }

    fn find_lowpoints(&self) -> Vec<u8> {
        (0..N)
            .flat_map(|row| {
                (0..M).filter_map(move |col| {
                    Some(self.0[row][col]).filter(|value| {
                        [(-1i32, 0i32), (1i32, 0i32), (0i32, -1i32), (0i32, 1i32)]
                            .iter()
                            .map(|(dx, dy)| (row as i32 + dx, col as i32 + dy))
                            .filter(|(x, y)| {
                                x >= &0 && y >= &0 && x < &(N as i32) && y < &(M as i32)
                            })
                            .map(|(x, y)| self.0[x as usize][y as usize])
                            .all(|neighbor| value < &neighbor)
                    })
                })
            })
            .collect()
    }

    fn find_basins(&self) -> Vec<Vec<(usize, usize)>> {
        let visited: RefCell<HashSet<(usize, usize)>> = RefCell::new(HashSet::new());
        (0..N)
            .flat_map(|row| {
                let visited = &visited;
                (0..M).filter_map(move |col| {
                    let mut coords = Vec::new();
                    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                    queue.push_back((row, col));
                    while let Some((walk_row, walk_col)) = queue.pop_front() {
                        if visited.borrow().contains(&(walk_row, walk_col))
                            || self.0[walk_row][walk_col] == 9
                        {
                            continue;
                        }
                        coords.push((walk_row, walk_col));
                        visited.borrow_mut().insert((walk_row, walk_col));
                        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let next_row = match dr {
                                -1 => walk_row.checked_sub(1),
                                _ => walk_row.checked_add(dr as usize).filter(|r| r < &N),
                            };
                            let next_col = match dc {
                                -1 => walk_col.checked_sub(1),
                                _ => walk_col.checked_add(dc as usize).filter(|c| c < &M),
                            };
                            if let (Some(next_row), Some(next_col)) = (next_row, next_col) {
                                queue.push_back((next_row, next_col))
                            }
                        }
                    }
                    if !coords.is_empty() {
                        Some(coords)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Map;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn test_find_lowpoints() {
        let map = Map([
            [2u8, 1u8, 9u8, 9u8, 9u8, 4u8, 3u8, 2u8, 1u8, 0u8],
            [3u8, 9u8, 8u8, 7u8, 8u8, 9u8, 4u8, 9u8, 2u8, 1u8],
            [9u8, 8u8, 5u8, 6u8, 7u8, 8u8, 9u8, 8u8, 9u8, 2u8],
            [8u8, 7u8, 6u8, 7u8, 8u8, 9u8, 6u8, 7u8, 8u8, 9u8],
            [9u8, 8u8, 9u8, 9u8, 9u8, 6u8, 5u8, 6u8, 7u8, 8u8],
        ]);
        let lowpoints = map.find_lowpoints();
        assert_eq!(vec![1, 0, 5, 5], lowpoints);
    }

    #[test]
    fn test_find_basins() {
        let map = Map([
            [2u8, 1u8, 9u8, 9u8, 9u8, 4u8, 3u8, 2u8, 1u8, 0u8],
            [3u8, 9u8, 8u8, 7u8, 8u8, 9u8, 4u8, 9u8, 2u8, 1u8],
            [9u8, 8u8, 5u8, 6u8, 7u8, 8u8, 9u8, 8u8, 9u8, 2u8],
            [8u8, 7u8, 6u8, 7u8, 8u8, 9u8, 6u8, 7u8, 8u8, 9u8],
            [9u8, 8u8, 9u8, 9u8, 9u8, 6u8, 5u8, 6u8, 7u8, 8u8],
        ]);
        let expected = vec![
            hashset![(0, 0), (0, 1), (1, 0)],
            hashset![
                (0, 5),
                (0, 6),
                (0, 7),
                (0, 8),
                (0, 9),
                (1, 6),
                (1, 8),
                (1, 9),
                (2, 9)
            ],
            hashset![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (4, 1)
            ],
            hashset![
                (2, 7),
                (3, 6),
                (3, 7),
                (3, 8),
                (4, 5),
                (4, 6),
                (4, 7),
                (4, 8),
                (4, 9)
            ],
        ];
        let basins: Vec<HashSet<_>> = map
            .find_basins()
            .into_iter()
            .map(|basin| basin.into_iter().collect())
            .collect();
        assert_eq!(expected, basins);
    }
}
