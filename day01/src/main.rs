use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long, default_value = "0")]
    window: usize,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let numbers: Vec<i64> = input
        .split('\n')
        .map(i64::from_str)
        .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;
    let numbers = if opts.window > 0 {
        println!("Using window size {}", opts.window);
        sum_sliding_window(&numbers, opts.window)
    } else {
        numbers
    };
    let increasing = count_increasing(&numbers);
    println!("Number of increasing measurements: {}", increasing);
    Ok(())
}

fn sum_sliding_window(numbers: &[i64], size: usize) -> Vec<i64> {
    numbers.windows(size).map(|it| it.iter().sum()).collect()
}

fn count_increasing(numbers: &[i64]) -> usize {
    numbers
        .windows(2)
        .filter(|slice| slice[0] < slice[1])
        .count()
}

#[cfg(test)]
mod test {
    use crate::{count_increasing, sum_sliding_window};

    #[test]
    fn test_count_increasing() {
        let numbers = [1, 2, 1, 3, 5, 6, 7, 8, 7, 9];
        assert_eq!(7, count_increasing(&numbers));
    }

    #[test]
    fn test_sum_sliding_window() {
        let numbers = [1, 2, 1, 3, 5, 6, 7, 8, 7, 9];

        assert_eq!(
            vec![4, 6, 9, 14, 18, 21, 22, 24],
            sum_sliding_window(&numbers, 3)
        );
    }
}
