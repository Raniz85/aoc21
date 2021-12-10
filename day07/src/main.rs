use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    expensive: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let numbers = input
        .split(',')
        .filter_map(|n| match n.trim() {
            "" => None,
            n => Some(u32::from_str(n)),
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let (target, score) = optimize(&numbers, opts.expensive);
    println!("Best target is {} with a fuel cost of {}", target, score);
    Ok(())
}

fn optimize(numbers: &[u32], expensive: bool) -> (u32, u32) {
    let min = numbers.iter().min().cloned().unwrap();
    let max = numbers.iter().max().cloned().unwrap();
    (min..=max)
        .map(|t| {
            (
                t,
                calc_fuel(
                    numbers,
                    t,
                    if expensive {
                        expensive_cost_function
                    } else {
                        cheap_cost_function
                    },
                ),
            )
        })
        .min_by_key(|(_, score)| *score)
        .unwrap()
}

fn cheap_cost_function(a: u32, b: u32) -> u32 {
    (a as i64 - b as i64).abs() as u32
}

fn expensive_cost_function(a: u32, b: u32) -> u32 {
    let distance = cheap_cost_function(a, b);
    distance * (distance + 1) / 2
}

fn calc_fuel(numbers: &[u32], target: u32, cost_function: fn(u32, u32) -> u32) -> u32 {
    numbers.iter().map(|n| cost_function(*n, target)).sum()
}

#[cfg(test)]
mod test {
    use crate::{cheap_cost_function, expensive_cost_function, optimize};
    use yare::parameterized;

    #[parameterized{
        one = {16, 2, 14},
        two = {1, 2, 1},
        three = {2, 2, 0},
        four = {0, 2, 2},
        five = {4, 2, 2},
        six = {7, 2, 5},
        seven = {14, 2, 12},
    }]
    fn test_cheap_cost_function(a: u32, b: u32, expected: u32) {
        assert_eq!(expected, cheap_cost_function(a, b,));
    }

    #[parameterized{
    one = {16, 5, 66},
    two = {1, 5, 10},
    three = {2, 5, 6},
    four = {0, 5, 15},
    five = {4, 5, 1},
    six = {7, 5, 3},
    seven = {14, 5, 45},
    }]
    fn test_expensive_cost_function(a: u32, b: u32, expected: u32) {
        assert_eq!(expected, expensive_cost_function(a, b,));
    }

    #[test]
    fn test_optimize() {
        let numbers = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let result = optimize(&numbers, false);
        assert_eq!((2, 37), result);
    }
}
