use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
}

struct Map<const N: usize, const M: usize>([[u8; M]; N]);

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let map: Map<100, 100> = Map::parse(input.split('\n'))?;
    let lowpoints = map.find_lowpoints();
    let risk_score = lowpoints.iter()
        .map(|n| *n as u32)
        .sum::<u32>() + lowpoints.len() as u32;
    println!("{}", risk_score);
    Ok(())
}

impl<const N: usize, const M: usize> Map<N, M> {
    fn parse<'a>(lines: impl Iterator<Item=&'a str>) -> Result<Map<N, M>> {
        Ok(Map(lines
            .map(|line| -> Result<[u8; M]> {
                Ok(line.chars()
                    .map(|c| match c {
                        '0'..='9' => Ok(c.to_digit(10).unwrap() as u8),
                        _ => Err(anyhow!("Invalid integer {}", c)),
                    })
                    .collect::<Result<Vec<u8>>>()?
                    .try_into()
                    .map_err(|v: Vec<u8>| anyhow!("Expected {} columns but was only {}", M, v.len()))?)
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|v: Vec<[u8; M]>| anyhow!("Expected {} rows but was only {}", N, v.len()))?))
    }

    fn find_lowpoints(&self) -> Vec<u8> {
        (0..N)
            .flat_map(|row| {
                (0..M).filter_map(move |col| {
                    Some(self.0[row][col]).filter(|value| {
                        [(-1i32, 0i32), (1i32, 0i32), (0i32, -1i32), (0i32, 1i32)]
                            .iter()
                            .map(|(dx, dy)| (row as i32 + dx, col as i32 + dy))
                            .filter(|(x, y)| x >= &0 && y >= &0 && x < &(N as i32) && y < &(M as i32))
                            .map(|(x, y)| self.0[x as usize][y as usize])
                            .all(|neighbor| {
                                value < &neighbor
                            })
                    })
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Map;

    #[test]
    fn test_find_lowpoints() {
        let map = Map([
                      [2u8, 1u8, 9u8, 9u8, 9u8, 4u8, 3u8, 2u8, 1u8, 0u8, ],
                      [3u8, 9u8, 8u8, 7u8, 8u8, 9u8, 4u8, 9u8, 2u8, 1u8, ],
                      [9u8, 8u8, 5u8, 6u8, 7u8, 8u8, 9u8, 8u8, 9u8, 2u8, ],
                      [8u8, 7u8, 6u8, 7u8, 8u8, 9u8, 6u8, 7u8, 8u8, 9u8, ],
                      [9u8, 8u8, 9u8, 9u8, 9u8, 6u8, 5u8, 6u8, 7u8, 8u8, ],
        ]);
        let lowpoints = map.find_lowpoints();
        assert_eq!(vec![1, 0, 5, 5], lowpoints);
    }
}
