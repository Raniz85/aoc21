use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    synch: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let (steps, flashes) = Field::<10, 10>::parse(input.split('\n'))?
        .run(if opts.synch { usize::MAX } else { 100 }, opts.synch);
    println!("Number of flashes: {} in {} steps", flashes, steps);
    Ok(())
}

#[derive(Debug)]
struct Field<const M: usize, const N: usize>([[u8; N]; M]);

impl<const M: usize, const N: usize> Field<M, N> {
    pub fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Field<N, M>> {
        Ok(Field(
            lines
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
                })?,
        ))
    }

    fn flash(&mut self, x: usize, y: usize) -> u64 {
        let mut flashes = 1;
        for nx in x.saturating_sub(1)..=(x + 1).min(M - 1) {
            for ny in y.saturating_sub(1)..=(y + 1).min(N - 1) {
                self.0[nx][ny] = self.0[nx][ny].saturating_add(1);
                if self.0[nx][ny] == 10 {
                    flashes += self.flash(nx, ny);
                }
            }
        }
        flashes
    }

    pub fn run(mut self, steps: usize, end_at_synch: bool) -> (usize, u64) {
        let mut flashes = 0;
        for step in 0..steps {
            let mut step_flashes = 0;
            for x in 0..M {
                for y in 0..N {
                    self.0[x][y] = self.0[x][y].saturating_add(1);
                    if self.0[x][y] == 10 {
                        step_flashes += self.flash(x, y);
                    }
                }
            }
            for x in 0..M {
                for y in 0..N {
                    if self.0[x][y] > 9 {
                        self.0[x][y] = 0;
                    }
                }
            }
            flashes += step_flashes;
            if end_at_synch && step_flashes as usize == N * M {
                return (step, flashes);
            }
        }
        (steps, flashes)
    }
}

#[cfg(test)]
mod test {
    use crate::Field;

    const LARGE_FIELD: [[u8; 10]; 10] = [
        [5u8, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2u8, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5u8, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6u8, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6u8, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4u8, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2u8, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6u8, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4u8, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5u8, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];

    #[test]
    fn test_small_run() {
        let field = Field([
            [1u8, 1, 1, 1, 1],
            [1u8, 9, 9, 9, 1],
            [1u8, 9, 1, 9, 1],
            [1u8, 9, 9, 9, 1],
            [1u8, 1, 1, 1, 1],
        ]);
        assert_eq!(9, field.run(2, false).1);
    }

    #[test]
    fn test_large_run_10() {
        let field = Field(LARGE_FIELD);
        assert_eq!(204, field.run(10, false).1);
    }

    #[test]
    fn test_large_run_100() {
        let field = Field(LARGE_FIELD);
        assert_eq!(1656, field.run(100, false).1);
    }

    #[test]
    fn test_large_synch() {
        let field = Field(LARGE_FIELD);
        assert_eq!(194, field.run(usize::MAX, true).0);
    }
}
