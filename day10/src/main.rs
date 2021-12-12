use anyhow::{bail, Result};
use clap::Parser;
use im::Vector;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let score: u32 = input
        .split('\n')
        .filter_map(|line| find_first_syntax_error(line, Vector::new()).transpose())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|error| match error {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum();
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

fn find_first_syntax_error(line: &str, stack: Vector<char>) -> Result<Option<char>> {
    if line.is_empty() {
        return Ok(None);
    }
    match line.chars().next().unwrap() {
        open @ ('(' | '[' | '{' | '<') => {
            find_first_syntax_error(&line[1..], stack + vec![get_closing(open)?].into())
        }
        close @ (')' | ']' | '}' | '>') => {
            if Some(&close) == stack.last() {
                let split = stack.len() - 1;
                find_first_syntax_error(&line[1..], stack.split_at(split).0)
            } else {
                Ok(Some(close))
            }
        }
        illegal => bail!("Invalid character {}", illegal),
    }
}

#[cfg(test)]
mod test {
    use crate::find_first_syntax_error;
    use im::Vector;
    use yare::parameterized;

    #[parameterized{
        one = { "[({(<(())[]>[[{[]{<()<>>", None },
        two = { "{([(<{}[<>[]}>{[]{[(<()>", Some('}') },
        three = { "[[<[([]))<([[{}[[()]]]", Some(')') },
        four = { "[{[{({}]{}}([{[{{{}}([]", Some(']') },
        five = { "[<(<(<(<{}))><([]([]()", Some(')') },
        six = { "<{([([[(<>()){}]>(<<{{", Some('>') },
    }]
    fn test_find_first_syntax_error(line: &str, expected: Option<char>) {
        let actual = find_first_syntax_error(line, Vector::new());
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap());
    }
}
