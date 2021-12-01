use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;
use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    File::open("input")?.read_to_string(&mut input)?;
    let numbers: Vec<i64> = input.split('\n')
        .map(i64::from_str)
        .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;
    let windows = sum_sliding_window(&numbers, 3);
    let increasing = count_increasing(&windows);
    println!("Number of increasing measurements: {}", increasing);
    Ok(())
}

fn sum_sliding_window(numbers: &[i64], size: usize) -> Vec<i64> {
    let mut window = Vec::new();
    let mut result = Vec::new();
    for number in numbers {
        window.push(number);
        if window.len() == size {
            result.push(window.iter().cloned().sum());
            window.remove(0);
        }
    }
    result
}

fn count_increasing(numbers: &[i64]) -> i32 {
    let mut last: i64 = 0;
    let mut increasing = 0;
    let mut first = true;
    for number in numbers {
        if !first && *number > last {
            increasing += 1;
        }
        first = false;
        last = *number;
    }
    increasing
}

mod test {
    use crate::{count_increasing, sum_sliding_window};

    #[test]
    fn test_count_increasing() {
        let numbers = [
            1, 2, 1, 3, 5, 6, 7, 8, 7, 9
        ];
        assert_eq!(7, count_increasing(&numbers));
    }

    #[test]
    fn test_sum_sliding_window() {
        let numbers = [
            1, 2, 1, 3, 5, 6, 7, 8, 7, 9
        ];

        assert_eq!(vec![4, 6, 9, 14, 18, 21, 22, 24], sum_sliding_window(&numbers, 3));
    }
}