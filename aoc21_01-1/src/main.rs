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
    let increasing = count_increasing(&numbers);
    println!("Number of increasing measurements: {}", increasing);
    Ok(())
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
    use crate::count_increasing;

    #[test]
    fn test_count_increasing() {
        let numbers = [
            1, 2, 1, 3, 5, 6, 7, 8, 7, 9
        ];
        assert_eq!(7, count_increasing(&numbers));
    }
}