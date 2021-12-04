use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::ops::Shr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let lines = input.split('\n').collect::<Vec<&str>>();

    let (gamma, num_bits) = calc_gamma(&lines)?;
    let epsilon = calc_epsilon(gamma, num_bits);

    println!("gamma = {}, epsilon = {}, epsilon * gamma = {}", gamma, epsilon, epsilon * gamma);

    Ok(())
}

fn calc_epsilon(gamma: u32, num_bits: u8) -> u32 {
    !gamma & (u32::MAX.shr(32 - num_bits))
}

fn calc_gamma(lines: &[&str]) -> Result<(u32, u8)> {
    let bit_counts = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|b| match b {
                    '0' => Ok(0),
                    '1' => Ok(1),
                    _ => bail!("Invalid bit {}", b),
                })
                .collect::<Result<Vec<u32>>>()
        })
        .reduce(|a, b| match (a, b) {
            (Ok(a), Ok(b)) => Ok(a
                .iter()
                .zip(b.iter())
                .map(|(av, bv)| av + bv)
                .collect::<Vec<u32>>()),
            (_, Err(e)) => Err(e),
            (Err(e), _) => Err(e),
        })
        .unwrap_or_else(|| Err(anyhow!("No input")))?;
    let num_bits = bit_counts.len() as u8;
    let gamma = bit_counts
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| {
            let bit: u32 = if *v >= (lines.len() / 2) as u32 { 1 } else { 0 };
            bit << i
        })
        .sum();
    Ok((gamma, num_bits))
}

#[cfg(test)]
mod test {
    use crate::{calc_epsilon, calc_gamma};

    #[test]
    fn test_calc_gamma() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let result = calc_gamma(&input);
        assert!(result.is_ok());
        assert_eq!((22, 5), result.unwrap());
    }

    #[test]
    fn test_calc_epsilon() {
        let gamma = 22;
        let num_bits = 5;

        let epsilon = calc_epsilon(gamma, num_bits);
        assert_eq!(9, epsilon);
    }
}
