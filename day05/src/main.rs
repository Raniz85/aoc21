use std::collections::HashMap;
use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use itertools::Either;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Line(Point, Point);

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;

    let lines = input.split('\n')
        .map(Line::parse)
        .collect::<Result<Vec<_>>>()?;
    let twice_covered = get_covered_points(&lines).iter()
        .map(|(_, count)| count)
        .filter(|count| **count >= 2)
        .count();
    println!("{} points are covered more than twice", twice_covered);

    Ok(())
}

fn get_covered_points(lines: &[Line]) -> HashMap<Point, u32> {
    lines.iter()
        .filter_map(Line::get_points)
        .flatten()
        .fold(HashMap::new(), |mut map, point| {
            match map.get_mut(&point) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    map.insert(point, 1);
                }
            }
            map
        })
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y
        }
    }

    fn parse(s: &str) -> Result<Point> {
        let coords = s.split(',')
            .map(|p| p.trim())
            .map(|n| Ok(u32::from_str(n)?))
            .collect::<Result<Vec<_>>>()?;
        match coords.as_slice() {
            [x, y] => Ok(Point::new(*x, *y)),
            _ => Err(anyhow!("Invalid point {}", s)),
        }
    }
}

impl Line {
    fn parse(s: &str) -> Result<Line> {
        let parts = s.split("->")
            .collect::<Vec<_>>();
        match parts.as_slice() {
            [a, b] => Ok(Line(Point::parse(a)?, Point::parse(b)?)),
            _ => Err(anyhow!("Invalid line {}", s)),
        }
    }

    fn is_diagonal(&self) -> bool {
        self.0.x != self.1.x && self.0.y != self.1.y
    }

    fn get_points(&self) -> Option<Vec<Point>> {
        if self.0.x == self.1.x {
            Some(get_range_inclusive(self.0.y, self.1.y)
                .map(|y| Point::new(self.0.x, y))
                .collect())
        } else if self.0.y == self.1.y {
            Some(get_range_inclusive(self.0.x, self.1.x)
                .map(|x| Point::new(x, self.0.y))
                .collect())
        } else {
            None
        }
    }
}

fn get_range_inclusive(a: u32, b: u32) -> impl Iterator<Item=u32> {
    if b >= a {
        Either::Left(a..=b)
    } else {
        Either::Right((b..=a).rev())
    }
}

#[cfg(test)]
mod test {
    use yare::parameterized;
    use crate::{get_range_inclusive, get_covered_points, Line, Point};
    use maplit::hashmap;

    #[parameterized{
        forward = {0, 2, vec![0, 1, 2]},
        reverse = {2, 0, vec![2, 1, 0]},
        single = {0, 0, vec![0]},
    }]
    fn test_get_range_inclusive(a: u32, b:u32, expected: Vec<u32>) {
        let actual = get_range_inclusive(a, b)
            .collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[parameterized{
    ok1 = { "5,6 -> 5,6", Some((5, 6, 5, 6)) },
    ok2 = { "5,6 -> 5,6", Some((5, 6, 5, 6)) },
    bad1 = { "5,", None },
    bad2 = { ",", None },
    bad3 = { ",6", None },
    bad4 = { "", None },
    bad5 = { "foo", None },
    }]
    fn test_parse_line(source: &str, expected: Option<(u32, u32, u32, u32)>) {
        let result = Line::parse(source);
        match expected {
            Some((ax, ay, bx, by)) => {
                assert!(result.is_ok());
                assert_eq!(Line(Point::new(ax, ay), Point::new(bx, by)), result.unwrap());
            },
            None => assert!(result.is_err()),
        }
    }

    #[parameterized{
        ok1 = { "5,6", Some((5, 6)) },
        ok2 = { "5 ,6", Some((5, 6)) },
        ok3 = { "5 , 6", Some((5, 6)) },
        ok4 = { "5, 6", Some((5, 6)) },
        bad1 = { "5,", None },
        bad2 = { ",", None },
        bad3 = { ",6", None },
        bad4 = { "", None },
        bad5 = { "foo", None },
    }]
    fn test_parse_point(source: &str, expected: Option<(u32, u32)>) {
        let result = Point::parse(&source);
        match expected {
            Some((x, y)) => {
                assert!(result.is_ok());
                assert_eq!(Point::new(x, y), result.unwrap());
            },
            None => assert!(result.is_err()),
        }
    }

    #[parameterized(
        horizontal = { Point::new(5, 0), Point::new(5, 8), false },
        vertical = { Point::new(5, 0), Point::new(8, 0), false },
        diagonal = { Point::new(5, 0), Point::new(0, 5), true },
    )]
    fn test_is_diagonal(a: Point, b: Point, diagonal: bool) {
        let line = Line(a, b);

        assert_eq!(diagonal, line.is_diagonal());
    }

    #[test]
    fn test_get_line_points_horizontal() {
        let line = Line(Point::new(0, 5), Point::new(0, 8));
        let points = line.get_points();
        assert!(points.is_some());
        assert_eq!(vec![
            Point::new(0, 5),
            Point::new(0, 6),
            Point::new(0, 7),
            Point::new(0, 8),
        ], points.unwrap());
    }

    #[test]
    fn test_get_line_points_vertical() {
        let line = Line(Point::new(0, 5), Point::new(8, 5));
        let points = line.get_points();
        assert!(points.is_some());
        assert_eq!(vec![
            Point::new(0, 5),
            Point::new(1, 5),
            Point::new(2, 5),
            Point::new(3, 5),
            Point::new(4, 5),
            Point::new(5, 5),
            Point::new(6, 5),
            Point::new(7, 5),
            Point::new(8, 5),
        ], points.unwrap());
    }

    #[test]
    fn test_get_line_points_negative() {
        let line = Line(Point::new(2, 5), Point::new(0, 5));
        let points = line.get_points();
        assert!(points.is_some());
        assert_eq!(vec![
            Point::new(2, 5),
            Point::new(1, 5),
            Point::new(0, 5),
        ], points.unwrap());
    }

    #[test]
    fn test_get_line_points_diagonal() {
        let line = Line(Point::new(0, 5), Point::new(5, 0));
        let points = line.get_points();
        assert!(points.is_none());
    }

    #[test]
    fn test_get_covered_points() {
        let lines = [
            Line(Point::new(0, 5), Point::new(0, 2)),
            Line(Point::new(0, 5), Point::new(0, 1)),
            Line(Point::new(0, 5), Point::new(2, 5)),
            Line(Point::new(0, 3), Point::new(3, 0)), // This shouldn't contribute
        ];

        let expected = hashmap!{
            Point::new(0, 5) => 3,
            Point::new(0, 4) => 2,
            Point::new(0, 3) => 2,
            Point::new(0, 2) => 2,
            Point::new(0, 1) => 1,
            Point::new(1, 5) => 1,
            Point::new(2, 5) => 1,
        };

        let covered = get_covered_points(&lines);
        assert_eq!(expected, covered);
    }
}
