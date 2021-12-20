use anyhow::{bail, Result};
use clap::Parser;
use im::Vector;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long)]
    fix: bool,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let analysis = input
        .split('\n')
        .map(|line| analyze_line(line, Vector::new()))
        .collect::<Result<Vec<_>>>()?;
    let score: u64 = if opts.fix {
        let scores = analysis
            .into_iter()
            .filter(LineResult::is_incomplete)
            .map(LineResult::unwrap_incomplete)
            .map(|errors| {
                errors
                    .into_iter()
                    .map(|c| match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!(),
                    })
                    .fold(0, |score, point| score * 5 + point)
            })
            .sorted()
            .collect::<Vec<_>>();
        scores[scores.len() / 2]
    } else {
        analysis
            .into_iter()
            .filter(LineResult::is_syntax_error)
            .map(|error| match error.unwrap_syntax_error() {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            })
            .sum()
    };
    println!("{}", score);
    Ok(())
}

fn get_closing(c: char) -> Result<char> {
    Ok(match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => bail!("Invalid character {}", c),
    })
}

#[derive(Debug, Eq, PartialEq)]
enum LineResult {
    Good,
    SyntaxError(char),
    Incomplete(Vector<char>),
}

impl LineResult {
    fn is_good(&self) -> bool {
        match self {
            LineResult::Good => true,
            _ => false,
        }
    }

    fn is_syntax_error(&self) -> bool {
        match self {
            LineResult::SyntaxError(_) => true,
            _ => false,
        }
    }

    fn is_incomplete(&self) -> bool {
        match self {
            LineResult::Incomplete(_) => true,
            _ => false,
        }
    }

    fn unwrap_syntax_error(self) -> char {
        match self {
            LineResult::SyntaxError(c) => c,
            _ => panic!("Not a syntax error"),
        }
    }

    fn unwrap_incomplete(self) -> Vector<char> {
        match self {
            LineResult::Incomplete(chars) => chars,
            _ => panic!("Not incomplete"),
        }
    }
}

fn analyze_line(line: &str, stack: Vector<char>) -> Result<LineResult> {
    if line.is_empty() {
        if stack.is_empty() {
            return Ok(LineResult::Good);
        }
        return Ok(LineResult::Incomplete(stack.into_iter().rev().collect()));
    }
    match line.chars().next().unwrap() {
        open @ ('(' | '[' | '{' | '<') => {
            analyze_line(&line[1..], stack + vec![get_closing(open)?].into())
        }
        close @ (')' | ']' | '}' | '>') => {
            if Some(&close) == stack.last() {
                let split = stack.len() - 1;
                analyze_line(&line[1..], stack.split_at(split).0)
            } else {
                Ok(LineResult::SyntaxError(close))
            }
        }
        illegal => bail!("Invalid character {}", illegal),
    }
}

#[cfg(test)]
mod test {
    use crate::{analyze_line, LineResult};
    use im::Vector;
    use yare::parameterized;

    #[parameterized{
        one = { "[({(<(())[]>[[{[]{<()<>>}}]])})]", LineResult::Good },
        two = { "{([(<{}[<>[]}>{[]{[(<()>", LineResult::SyntaxError('}') },
        three = { "[[<[([]))<([[{}[[()]]]", LineResult::SyntaxError(')') },
        four = { "[{[{({}]{}}([{[{{{}}([]", LineResult::SyntaxError(']') },
        five = { "[<(<(<(<{}))><([]([]()", LineResult::SyntaxError(')') },
        six = { "<{([([[(<>()){}]>(<<{{", LineResult::SyntaxError('>') },
        seven = { "[(()[<>])]({[<{<<[]>>(", LineResult::Incomplete(")}>]})".chars().collect()) },
    }]
    fn test_find_first_syntax_error(line: &str, expected: LineResult) {
        let actual = analyze_line(line, Vector::new());
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap());
    }
}
