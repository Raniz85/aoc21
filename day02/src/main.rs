use anyhow::{bail, Result};
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
    println!(
        "Resulting position: ({}, {}) (={})",
        horizontal,
        vertical,
        horizontal * vertical
    );

    Ok(())
}

trait Navigation {
    fn handle_forward(&mut self, amount: i64);
    fn handle_down(&mut self, amount: i64);
    fn handle_up(&mut self, amount: i64);

    fn vertical_position(&self) -> i64;
    fn horizontal_position(&self) -> i64;
}

fn navigate(instructions: &[&str], navigation: &mut dyn Navigation) -> Result<()> {
    for instruction in instructions {
        let parts = instruction.splitn(2, ' ').collect::<Vec<&str>>();
        let direction = parts
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("Invalid instruction {}", instruction))?;
        let amount = parts
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("Invalid instruction {}", instruction))?;
        let amount = i64::from_str(amount)?;
        match *direction {
            "forward" => navigation.handle_forward(amount),
            "up" => navigation.handle_up(amount),
            "down" => navigation.handle_down(amount),
            _ => bail!("Unknown direction {}", direction),
        }
    }
    Ok(())
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

#[cfg(test)]
mod test {
    use crate::{navigate, AimNavigation, NaiveNavigation, Navigation};

    struct RecordingNavigation {
        pub instructions: Vec<(String, i64)>,
    }

    impl Navigation for RecordingNavigation {
        fn handle_forward(&mut self, amount: i64) {
            self.instructions.push(("forward".to_owned(), amount))
        }

        fn handle_down(&mut self, amount: i64) {
            self.instructions.push(("down".to_owned(), amount))
        }

        fn handle_up(&mut self, amount: i64) {
            self.instructions.push(("up".to_owned(), amount))
        }

        fn vertical_position(&self) -> i64 {
            0
        }

        fn horizontal_position(&self) -> i64 {
            0
        }
    }

    impl Default for RecordingNavigation {
        fn default() -> Self {
            RecordingNavigation {
                instructions: Vec::new(),
            }
        }
    }

    #[test]
    fn test_navigate() {
        let instructions = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        // Use NaiveNavigation to test that all instructions are read correctly
        let mut navigation = RecordingNavigation::default();

        let result = navigate(&instructions, &mut navigation);
        assert!(result.is_ok());

        let expected_instructions = vec![
            ("forward".to_owned(), 5),
            ("down".to_owned(), 5),
            ("forward".to_owned(), 8),
            ("up".to_owned(), 3),
            ("down".to_owned(), 8),
            ("forward".to_owned(), 2),
        ];
        assert_eq!(expected_instructions, navigation.instructions);
    }

    #[test]
    fn test_navigate_naive() {
        let mut navigation = NaiveNavigation::default();

        navigation.handle_forward(5);
        navigation.handle_down(5);
        navigation.handle_forward(0);
        navigation.handle_up(3);
        navigation.handle_down(8);
        navigation.handle_forward(2);

        assert_eq!(15, navigation.horizontal_position());
        assert_eq!(10, navigation.vertical_position());
    }

    #[test]
    fn test_navigate_aim() {
        let mut navigation = AimNavigation::default();

        navigation.handle_forward(5);
        navigation.handle_down(5);
        navigation.handle_forward(0);
        navigation.handle_up(3);
        navigation.handle_down(8);
        navigation.handle_forward(2);

        assert_eq!(15, navigation.horizontal_position());
        assert_eq!(60, navigation.vertical_position());
    }
}
