use anyhow::{anyhow, Result};
use clap::Parser;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    solve: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let signals = input
        .split('\n')
        .map(Signal::parse)
        .collect::<Result<Vec<_>>>()?;
    if opts.solve {
        let sum: u64 = signals
            .iter()
            .map(|signal| signal.get_output() as u64)
            .sum();
        println!("{}", sum);
    } else {
        let count: u32 = signals.iter().map(Signal::known_output_digits).sum();
        println!("{}", count);
    }
    Ok(())
}

#[derive(Debug)]
struct Signal<'a> {
    pattern: [Digit<'a>; 10],
    output: [Digit<'a>; 4],
}

#[derive(Clone, Copy, Debug)]
struct Digit<'a>(&'a str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Segment {
    Top = 0,
    TopLeft = 1,
    TopRight = 2,
    Middle = 3,
    BottomLeft = 4,
    BottomRight = 5,
    Bottom = 6,
}

impl<'a> Signal<'a> {
    fn parse(line: &str) -> Result<Signal> {
        match line.split(" | ").collect::<Vec<_>>().as_slice() {
            [pattern, output] => Ok(Signal {
                pattern: pattern
                    .split(' ')
                    .map(|d| Digit(d))
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|v: Vec<Digit>| {
                        anyhow!("Expected vec of size 10 but was {}", v.len())
                    })?,
                output: output
                    .split(' ')
                    .map(|d| Digit(d))
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|v: Vec<Digit>| {
                        anyhow!("Expected vec of size 4 but was {}", v.len())
                    })?,
            }),
            _ => Err(anyhow!("Invalid signal {}", line)),
        }
    }

    fn known_output_digits(&self) -> u32 {
        self.output
            .iter()
            .filter(|digit| [2, 3, 4, 7].contains(&digit.0.len()))
            .count() as u32
    }

    pub fn get_output(&self) -> u32 {
        let wiring = self.deduce_wiring();
        self.output
            .iter()
            .map(|d| d.get_value(&wiring))
            .rev()
            .enumerate()
            .map(|(index, value)| 10u32.pow(index as u32) * value as u32)
            .sum()
    }

    pub(crate) fn deduce_wiring(&self) -> [char; 7] {
        self.deduce_wiring_rec(['x'; 7])
    }

    fn deduce_wiring_rec(&self, wiring: [char; 7]) -> [char; 7] {
        if !wiring.contains(&'x') {
            return wiring;
        }
        let wiring = [
            Segment::Top,
            Segment::TopLeft,
            Segment::TopRight,
            Segment::Middle,
            Segment::BottomLeft,
            Segment::BottomRight,
            Segment::Bottom,
        ]
        .into_iter()
        .map(|segment| {
            let by_digit = self.get_possible_wires_by_digit(segment, &wiring);
            let by_frequency = self.get_possible_wires_by_frequency(segment);
            if by_digit.len() == 1 {
                by_digit
            } else if by_frequency.len() == 1 {
                by_frequency
            } else {
                by_digit
                    .union(&by_frequency)
                    .filter(|c| !wiring.contains(c))
                    .cloned()
                    .collect::<HashSet<char>>()
            }
        })
        .enumerate()
        .map(|(index, chars)| {
            if wiring[index] != 'x' {
                wiring[index]
            } else if chars.len() == 1 {
                chars.into_iter().next().unwrap()
            } else {
                'x'
            }
        })
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();
        self.deduce_wiring_rec(wiring)
    }

    fn get_possible_wires_by_frequency(&self, segment: Segment) -> HashSet<char> {
        ('a'..='g')
            .filter(|c| {
                self.get_wire_frequency(*c)
                    == match segment {
                        Segment::Top => 8,
                        Segment::TopLeft => 6,
                        Segment::TopRight => 8,
                        Segment::Middle => 7,
                        Segment::BottomLeft => 4,
                        Segment::BottomRight => 9,
                        Segment::Bottom => 7,
                    }
            })
            .collect()
    }

    fn get_possible_wires_by_digit(&self, segment: Segment, wiring: &[char; 7]) -> HashSet<char> {
        self.pattern
            .iter()
            .filter(|digit| digit.may_activate_segment(segment, wiring))
            .fold(('a'..='g').collect(), |set, digit| {
                set.intersection(&digit.0.chars().collect())
                    .cloned()
                    .collect()
            })
    }

    fn get_wire_frequency(&self, c: char) -> usize {
        self.pattern.iter().filter(|d| d.0.contains(c)).count()
    }
}

impl<'a> Digit<'a> {
    fn may_activate_segment(&self, segment: Segment, wiring: &[char; 7]) -> bool {
        self.get_possible_numbers(wiring)
            .into_iter()
            .flat_map(|n| self.get_segments(n))
            .any(|s| s == segment)
    }

    fn get_possible_numbers(&self, wiring: &[char; 7]) -> HashSet<u8> {
        match self.0.len() {
            2 => vec![1],
            3 => vec![7],
            4 => vec![4],
            5 => [2, 3, 5]
                .into_iter()
                .filter(|n| match n {
                    2 => {
                        !(self.0.contains(wiring[Segment::TopLeft as usize])
                            || self.0.contains(wiring[Segment::BottomRight as usize]))
                    }
                    3 => {
                        !(self.0.contains(wiring[Segment::TopLeft as usize])
                            || self.0.contains(wiring[Segment::BottomLeft as usize]))
                    }
                    5 => {
                        !(self.0.contains(wiring[Segment::TopRight as usize])
                            || self.0.contains(wiring[Segment::BottomLeft as usize]))
                    }
                    _ => panic!(),
                })
                .collect(),
            6 => [0, 6, 9]
                .into_iter()
                .filter(|n| match n {
                    0 => !self.0.contains(wiring[Segment::Middle as usize]),
                    6 => !self.0.contains(wiring[Segment::TopRight as usize]),
                    9 => !self.0.contains(wiring[Segment::BottomLeft as usize]),
                    _ => panic!(),
                })
                .collect(),
            7 => vec![8],
            _ => panic!("Impossible digit {}", self.0),
        }
        .into_iter()
        .collect()
    }

    fn get_segments(&self, number: u8) -> Vec<Segment> {
        match number {
            0 => vec![
                Segment::Top,
                Segment::TopLeft,
                Segment::TopRight,
                Segment::BottomLeft,
                Segment::BottomRight,
                Segment::Bottom,
            ],
            1 => vec![Segment::TopRight, Segment::BottomRight],
            2 => vec![
                Segment::Top,
                Segment::TopRight,
                Segment::Middle,
                Segment::BottomLeft,
                Segment::Bottom,
            ],
            3 => vec![
                Segment::Top,
                Segment::TopRight,
                Segment::Middle,
                Segment::BottomRight,
                Segment::Bottom,
            ],
            4 => vec![
                Segment::TopLeft,
                Segment::TopRight,
                Segment::Middle,
                Segment::BottomRight,
            ],
            5 => vec![
                Segment::Top,
                Segment::TopLeft,
                Segment::Middle,
                Segment::BottomRight,
                Segment::Bottom,
            ],
            6 => vec![
                Segment::Top,
                Segment::TopLeft,
                Segment::Middle,
                Segment::BottomLeft,
                Segment::BottomRight,
                Segment::Bottom,
            ],
            7 => vec![Segment::Top, Segment::TopRight, Segment::BottomRight],
            8 => vec![
                Segment::Top,
                Segment::TopLeft,
                Segment::TopRight,
                Segment::Middle,
                Segment::BottomLeft,
                Segment::BottomRight,
                Segment::Bottom,
            ],
            9 => vec![
                Segment::Top,
                Segment::TopLeft,
                Segment::TopRight,
                Segment::Middle,
                Segment::BottomRight,
                Segment::Bottom,
            ],
            _ => panic!("Invalid number {}", number),
        }
    }

    fn get_value(&self, wiring: &[char; 7]) -> u8 {
        (0..=9)
            .find(|n| {
                let segments = self.get_segments(*n);
                segments.len() == self.0.len()
                    && segments
                        .into_iter()
                        .all(|segment| self.0.contains(wiring[segment as usize]))
            })
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{Digit, Signal};
    use yare::parameterized;

    #[parameterized{
        one = { ["ab", "abcdef", "abcdef", "abcde"], 1},
        two = { ["ab", "abc", "abcdef", "abcdef"], 2},
        three = { ["ab", "abcd", "abc", "abcdef"], 3},
        four = { ["ab", "abcd", "abc", "abcdefg"], 4},
    }]
    fn test_known_output_digits(output_digits: [&str; 4], expected: u32) {
        let pattern = (0..10)
            .map(|_| Digit("a"))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let output = output_digits.map(|d| Digit(d));
        let signal = Signal { pattern, output };

        assert_eq!(expected, signal.known_output_digits())
    }

    #[parameterized{
        zero = { "abcefg", 0 },
        one = { "cf", 1 },
        two = { "acdeg", 2 },
        three = { "acdfg", 3 },
        four = { "bcdf", 4 },
        five = { "abdfg", 5 },
        six = { "abdefg", 6 },
        seven = { "acf", 7 },
        eight = { "abcdefg", 8 },
        nine = { "abcdfg", 9 },
    }]
    fn test_get_value() {
        let _wiring: [char; 7] = ('a'..='g').collect::<Vec<_>>().try_into().unwrap();
    }

    #[test]
    fn test_deduce_wiring() {
        let signal = Signal::parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();
        let wiring = signal.deduce_wiring();
        assert_eq!(['d', 'e', 'a', 'f', 'g', 'b', 'c'], wiring);
    }

    #[test]
    fn test_get_single_output() {
        let signal = Signal::parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )
        .unwrap();
        assert_eq!(5353, signal.get_output());
    }
}
