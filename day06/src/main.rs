use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long, default_value = "80")]
    days: u32,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let starts = input
        .split(",")
        .map(|n| Ok(u32::from_str(n.trim())?))
        .collect::<Result<Vec<_>>>()?;
    let fish = count_fishes(&starts, opts.days);
    println!("After {} days there will be {} fish", opts.days, fish);
    Ok(())
}

fn count_fishes(starts: &[u32], days: u32) -> u64 {
    starts.iter().map(|s| count_fish(*s, days)).sum()
}

fn count_fish(start: u32, days: u32) -> u64 {
    let mut production_days = (start..days)
        .step_by(7)
        .map(|day| (day, 1))
        .collect::<HashMap<_, _>>();
    let mut sum = 1;
    for day in (start..days) {
        if let Some(amount) = production_days.get(&day) {
            let amount = *amount;
            for production_day in ((day + 9)..days).step_by(7) {
                match production_days.get_mut(&production_day) {
                    Some(existing_amount) => {
                        *existing_amount += amount;
                    }
                    None => {
                        production_days.insert(production_day, amount);
                    }
                }
            }
            sum += amount;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::{count_fish, count_fishes};
    use yare::parameterized;

    #[parameterized{
        short1 = { 0, 8, 3},
        short2 = { 7, 8, 2},
        short3 = { 5, 8, 2},
        medium1 = { 3, 18, 5},
        medium2 = { 4, 18, 4},
        medium3 = { 2, 18, 5},
        medium4 = { 1, 18, 7},
    }]
    fn test_count_fish(start: u32, days: u32, expected: u32) {
        assert_eq!(expected, count_fish(start, days));
    }

    #[parameterized{
        days18 = { 18, 26 },
        days80 = { 80, 5934 },
    }]
    fn test_count_fishes(days: u32, expected: u32) {
        let starts = [3, 4, 3, 1, 2];

        assert_eq!(expected, count_fishes(&starts, days));
    }
}
