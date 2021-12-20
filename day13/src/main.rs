use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(short, long, default_value = "1")]
    folds: usize,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let input = input.split('\n').collect_vec();
    let (image, folds) = Image::parse(&input)?;
    let folded_image = folds.iter()
        .filter(|line| !line.is_empty())
        .take(opts.folds)
        .fold(Ok(image), |image, fold| {
            let (direction, line) = fold.splitn(2, '=').collect_tuple()
                .ok_or_else(|| anyhow!("Invalid fold {}", fold))?;
            let line = usize::from_str(line)?;
            match direction {
                "fold along y" => image.map(|image| image.fold_horizontal(line)),
                "fold along x" => image.map(|image| image.fold_vertical(line)),
                _ => bail!("Invalid fold {}", fold),
            }
        })?;
    println!("{}", folded_image.paint());
    println!("Folded image has {} dots", folded_image.dots());
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Image {
    pixels: BTreeSet<Pixel>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Pixel(usize, usize);

impl Image {
    fn parse<'a>(lines: &'a[&'a str]) -> Result<(Image, &'a[&'a str])> {
        let mut pixels = BTreeSet::new();
        for (index, line) in lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            } else if line.starts_with("fold") {
                return Ok((Image {
                    pixels,
                }, &lines[index..]))
            }
            let (x, y) = line.splitn(2, ',').collect_tuple()
                .ok_or_else(|| anyhow!("Invalid coordinate {}", line))?;
            let x = usize::from_str(x)?;
            let y = usize::from_str(y)?;
            pixels.insert(Pixel(x, y));
        }
        bail!("EOL reached before any folds")
    }

    fn fold_horizontal(&self, y: usize) -> Image {
        println!("Folding horizontally along {}", y);
        Image {
            pixels: self.pixels.iter()
                .map(|p| p.fold_horizontal(y))
                .collect()
        }
    }

    fn fold_vertical(&self, x: usize) -> Image {
        println!("Folding vertically along {}", x);
        Image {
            pixels: self.pixels.iter()
                .map(|p| p.fold_vertical(x))
                .collect()
        }
    }

    fn dots(&self) -> usize {
        self.pixels.len()
    }

    fn paint(&self) -> String {
        let width = self.pixels.iter()
            .map(|p| p.0)
            .max().unwrap() + 1;
        let height = self.pixels.iter()
            .map(|p| p.1)
            .max().unwrap() + 1;
        let mut painting = (0..height)
            .map(|_| vec!["."; width])
            .collect_vec();
        self.pixels.iter()
            .for_each(|pixel| painting[pixel.1][pixel.0] = "#");
        painting.iter()
            .map(|row| row.join("") + "\n")
            .collect()
    }
}

impl Pixel {
    fn fold_horizontal(&self, y: usize) -> Pixel {
        if self.1 > y {
            Pixel(self.0, 2 * y - self.1)
        } else {
            self.clone()
        }
    }

    fn fold_vertical(&self, x: usize) -> Pixel {
        if self.0 > x {
            Pixel(2 * x - self.0, self.1)
        } else {
            self.clone()
        }
    }
}

impl PartialOrd for Pixel {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1)
            .then(self.0.cmp(&other.0)))
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}


#[cfg(test)]
mod test {
    use crate::{Image, Pixel};

    #[test]
    fn test_parse() {
        let input = [
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];
        let expected = original_image();

        let result = Image::parse(&input);
        assert!(result.is_ok());

        let (image, folds) = result.unwrap();
        assert_eq!(&["fold along y=7", "fold along x=5"], folds);
        assert_eq!(expected, image);
    }

    #[test]
    fn test_pixel_fold_vertical() {
        let pixel = Pixel(6, 0);
        assert_eq!(Pixel(4, 0), pixel.fold_vertical(5));
    }

    #[test]
    fn test_pixel_fold_horizontal() {
        let pixel = Pixel(0, 6);
        assert_eq!(Pixel(0, 4), pixel.fold_horizontal(5));
    }

    #[test]
    fn test_fold_vertical() {
        let image = original_image();

        let expected = folded_image();

        let folded = image.fold_horizontal(7);

        assert_eq!(expected, folded)
    }

    #[test]
    fn test_fold_horizontal() {
        let image = folded_image();

        let expected = twice_folded_image();

        let folded = image.fold_vertical(5);

        assert_eq!(expected, folded)
    }

    #[test]
    fn test_dots() {
        let image = folded_image();

        assert_eq!(17, image.dots());
    }

    fn original_image() -> Image {
        Image {
            pixels: [
                Pixel(6,10),
                Pixel(0,14),
                Pixel(9,10),
                Pixel(0,3),
                Pixel(10,4),
                Pixel(4,11),
                Pixel(6,0),
                Pixel(6,12),
                Pixel(4,1),
                Pixel(0,13),
                Pixel(10,12),
                Pixel(3,4),
                Pixel(3,0),
                Pixel(8,4),
                Pixel(1,10),
                Pixel(2,14),
                Pixel(8,10),
                Pixel(9,0)
            ].into_iter().collect(),
        }
    }

    fn folded_image() -> Image {
        Image {
            pixels: [
                Pixel(0,0),
                Pixel(2,0),
                Pixel(3,0),
                Pixel(6,0),
                Pixel(9,0),
                Pixel(0,1),
                Pixel(4,1),
                Pixel(6,2),
                Pixel(10,2),
                Pixel(0,3),
                Pixel(4,3),
                Pixel(1,4),
                Pixel(3,4),
                Pixel(6,4),
                Pixel(8,4),
                Pixel(9,4),
                Pixel(10,4),
            ].into_iter().collect(),
        }
    }

    fn twice_folded_image() -> Image {
        Image {
            pixels: [
                Pixel(0,0),
                Pixel(1,0),
                Pixel(2,0),
                Pixel(3,0),
                Pixel(4,0),
                Pixel(0,1),
                Pixel(4,1),
                Pixel(0,2),
                Pixel(4,2),
                Pixel(0,3),
                Pixel(4,3),
                Pixel(0,4),
                Pixel(1,4),
                Pixel(2,4),
                Pixel(3,4),
                Pixel(4,4),
            ].into_iter().collect(),
        }
    }

}
