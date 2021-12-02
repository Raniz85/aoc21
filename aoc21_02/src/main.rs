use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value="input")]
    input: String,
    #[clap(short, long)]
    aim: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut navigation: Box<dyn Navigation> = if opts.aim {
        Box::new(AimNavigation::default())
    } else {
        Box::new(NaiveNavigation::default())
    };
    navigate(&lines, navigation.as_mut())?;
    let horizontal = navigation.horizontal_position();
    let vertical = navigation.vertical_position();
    println!("Resulting position: ({}, {}) (={})", horizontal, vertical, horizontal * vertical);
    Ok(())
}

trait Navigation {
    fn handle_forward(&mut self, amount: i64);
    fn handle_down(&mut self, amount: i64);
    fn handle_up(&mut self, amount: i64);

    fn vertical_position(&self) -> i64;
    fn horizontal_position(&self) -> i64;
}

struct NaiveNavigation {
    horizontal: i64,
    vertical: i64,
}

impl Default for NaiveNavigation {
    fn default() -> Self {
        NaiveNavigation {
            horizontal: 0,
            vertical: 0,
        }
    }
}

impl Navigation for NaiveNavigation {
    fn handle_forward(&mut self, amount: i64) {
        self.horizontal += amount;
    }

    fn handle_down(&mut self, amount: i64) {
        self.vertical += amount;
    }

    fn handle_up(&mut self, amount: i64) {
        self.vertical -= amount;
    }

    fn vertical_position(&self) -> i64 {
        self.vertical
    }

    fn horizontal_position(&self) -> i64 {
        self.horizontal
    }
}

struct AimNavigation {
    naive: NaiveNavigation,
    aim: i64,
}

impl Default for AimNavigation {
    fn default() -> Self {
        AimNavigation {
            naive: NaiveNavigation::default(),
            aim: 0,
        }
    }
}

impl Navigation for AimNavigation {
    fn handle_forward(&mut self, amount: i64) {
        self.naive.horizontal += amount;
        self.naive.vertical += self.aim * amount;
    }

    fn handle_down(&mut self, amount: i64) {
        self.aim += amount;
    }

    fn handle_up(&mut self, amount: i64) {
        self.aim -= amount;
    }

    fn vertical_position(&self) -> i64 {
        self.naive.vertical_position()
    }

    fn horizontal_position(&self) -> i64 {
        self.naive.horizontal_position()
    }
}


fn navigate(instructions: &[&str], navigation: &mut dyn Navigation) -> Result<()> {
    for instruction in instructions {
        let parts = instruction.splitn(2, ' ').collect::<Vec<&str>>();
        let direction = parts[0];
        let amount = i64::from_str(parts[1])?;
        match direction {
            "forward" => navigation.handle_forward(amount),
            "up" => navigation.handle_up(amount),
            "down" => navigation.handle_down(amount),
            _ => bail!("Unknown direction {}", direction),
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{AimNavigation, NaiveNavigation, navigate, Navigation};

    #[test]
    fn test_navigate_naive() {
        let instructions = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let mut navigation = NaiveNavigation::default();
        let result = navigate(&instructions, &mut navigation);
        assert!(result.is_ok());
        assert_eq!(15, navigation.horizontal_position());
        assert_eq!(10, navigation.vertical_position());
    }

    #[test]
    fn test_navigate_aim() {
        let instructions = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let mut navigation = AimNavigation::default();
        let result = navigate(&instructions, &mut navigation);
        assert!(result.is_ok());
        assert_eq!(15, navigation.horizontal_position());
        assert_eq!(60, navigation.vertical_position());
    }

}