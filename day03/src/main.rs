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
    #[clap(short, long)]
    sieve: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let lines = input.split('\n').collect::<Vec<&str>>();

    if opts.sieve {
        let oxygen = sieve(&lines, 0, false)?;
        dbg!(oxygen);
        let co2 = sieve(&lines, 0, true)?;
        dbg!(co2);

        println!(
            "oxygen = {}, co2 = {}, oxygen * co2 = {}",
            oxygen,
            co2,
            oxygen * co2
        );
    } else {
        let (gamma, num_bits) = calc_gamma(&lines)?;
        let epsilon = calc_epsilon(gamma, num_bits);

        println!(
            "gamma = {}, epsilon = {}, epsilon * gamma = {}",
            gamma,
            epsilon,
            epsilon * gamma
        );
    }

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

fn sieve(lines: &[&str], index: usize, inverse: bool) -> Result<u32> {
    let ones = lines
        .iter()
        .filter(|line| &line[index..index + 1] == "1")
        .count();
    let zeroes = lines.len() - ones;
    let filter = if (inverse && ones < zeroes) || (!inverse && ones >= zeroes) {
        "1"
    } else {
        "0"
    };
    let mut candidates = lines.to_vec();
    candidates.retain(|line| &line[index..index + 1] == filter);
    match candidates.as_slice() {
        [hit] => Ok(u32::from_str_radix(hit, 2)?),
        hits => sieve(hits, index + 1, inverse),
    }
}

#[cfg(test)]
mod test {
    use crate::{calc_epsilon, calc_gamma, sieve};

    #[test]
    fn test_sieve() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let result = sieve(&input, 0, false);
        assert!(result.is_ok());
        assert_eq!(23, result.unwrap());

        let result = sieve(&input, 0, true);
        assert!(result.is_ok());
        assert_eq!(10, result.unwrap());
    }

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
