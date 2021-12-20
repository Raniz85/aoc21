use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raniz")]
struct Opts {
    #[clap(short, long, default_value = "input")]
    input: String,
    #[clap(long, default_value = "2")]
    iterations: usize,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut input = String::new();
    File::open(opts.input)?.read_to_string(&mut input)?;
    let lines = input.split('\n').collect::<Vec<_>>();

    let enhancer = ImageEnhancer::parse(lines[0])?;
    let image = Image::parse(&lines[2..])?;

    let image = (0..opts.iterations).fold(image, |image, _| enhancer.enhance(image));
    println!("{}", image.paint());

    println!(
        "Lit pixels in image of size {}x{}: {}",
        image.width,
        image.height,
        image.lit_pixels()
    );
    Ok(())
}

struct ImageEnhancer([bool; 512]);

#[derive(Debug, Eq, PartialEq)]
struct Image {
    pixels: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    default_pixel: bool,
}

impl ImageEnhancer {
    fn parse(line: impl AsRef<str>) -> Result<ImageEnhancer> {
        Ok(ImageEnhancer(
            line.as_ref()
                .chars()
                .map(|char| {
                    Ok(match char {
                        '#' => true,
                        '.' => false,
                        _ => bail!("Invalid pixel {}", char),
                    })
                })
                .collect::<Result<Vec<_>>>()?
                .try_into()
                .map_err(|vec: Vec<bool>| {
                    anyhow!("Expected line of length 512 but was {}", vec.len())
                })?,
        ))
    }

    fn enhance(&self, image: Image) -> Image {
        let pixels = (-1..(image.width as i64 + 1))
            .map(|x| {
                (-1..(image.height as i64 + 1))
                    .map(|y| self.0[image.sample_grid(x, y)])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let default_pixel = if image.default_pixel {
            self.0[511]
        } else {
            self.0[0]
        };
        Image {
            pixels,
            width: image.width + 2,
            height: image.height + 2,
            default_pixel,
        }
    }
}

impl Image {
    fn parse(lines: &[&str]) -> Result<Image> {
        let height = lines.len();
        let width = lines[0].len();
        let pixels = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|char| {
                        Ok(match char {
                            '#' => true,
                            '.' => false,
                            _ => bail!("Invalid pixel {}", char),
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .map(|line| match line {
                Ok(line) if line.len() == width => Ok(line),
                Ok(line) => Err(anyhow!(
                    "Expected line of size {} but was {}",
                    width,
                    line.len()
                )),
                error => error,
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Image {
            pixels,
            width,
            height,
            default_pixel: false,
        })
    }

    fn sample(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            self.default_pixel
        } else {
            self.pixels
                .get(x as usize)
                .and_then(|row| row.get(y as usize))
                .cloned()
                .unwrap_or(self.default_pixel)
        }
    }

    fn sample_grid(&self, x: i64, y: i64) -> usize {
        ((x - 1)..=(x + 1))
            .flat_map(|nx| ((y - 1)..=(y + 1)).map(move |ny| self.sample(nx, ny)))
            .rev()
            .enumerate()
            .map(|(pow, bit)| if bit { 1_usize } else { 0_usize } * 2_usize.pow(pow as u32))
            .sum()
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .filter(|pixel| **pixel)
            .count()
    }

    fn paint(&self) -> String {
        self.pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| if *pixel { "#" } else { "." })
                    .collect::<String>()
                    + "\n"
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{Image, ImageEnhancer};

    #[test]
    fn test_small() {
        let enhancer = ImageEnhancer([
            false, false, true, false, true, false, false, true, true, true, true, true, false,
            true, false, true, false, true, false, true, true, true, false, true, true, false,
            false, false, false, false, true, true, true, false, true, true, false, true, false,
            false, true, true, true, false, true, true, true, true, false, false, true, true, true,
            true, true, false, false, true, false, false, false, false, true, false, false, true,
            false, false, true, true, false, false, true, true, true, false, false, true, true,
            true, true, true, true, false, true, true, true, false, false, false, true, true, true,
            true, false, false, true, false, false, true, true, true, true, true, false, false,
            true, true, false, false, true, false, true, true, true, true, true, false, false,
            false, true, true, false, true, false, true, false, false, true, false, true, true,
            false, false, true, false, true, false, false, false, false, false, false, true, false,
            true, true, true, false, true, true, true, true, true, true, false, true, true, true,
            false, true, true, true, true, false, false, false, true, false, true, true, false,
            true, true, false, false, true, false, false, true, false, false, true, true, true,
            true, true, false, false, false, false, false, true, false, true, false, false, false,
            false, true, true, true, false, false, true, false, true, true, false, false, false,
            false, false, false, true, false, false, false, false, false, true, false, false, true,
            false, false, true, false, false, true, true, false, false, true, false, false, false,
            true, true, false, true, true, true, true, true, true, false, true, true, true, true,
            false, true, true, true, true, false, true, false, true, false, false, false, true,
            false, false, false, false, false, false, false, true, false, false, true, false, true,
            false, true, false, false, false, true, true, true, true, false, true, true, false,
            true, false, false, false, false, false, false, true, false, false, true, false, false,
            false, true, true, false, true, false, true, true, false, false, true, false, false,
            false, true, true, false, true, false, true, true, false, false, true, true, true,
            false, true, false, false, false, false, false, false, true, false, true, false, false,
            false, false, false, false, false, true, false, true, false, true, false, true, true,
            true, true, false, true, true, true, false, true, true, false, false, false, true,
            false, false, false, false, false, true, true, true, true, false, true, false, false,
            true, false, false, true, false, true, true, false, true, false, false, false, false,
            true, true, false, false, true, false, true, true, true, true, false, false, false,
            false, true, true, false, false, false, true, true, false, false, true, false, false,
            false, true, false, false, false, false, false, false, true, false, true, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, true, true, false, false, true, true, true, true, false, false, true, false,
            false, false, true, false, true, false, true, false, false, false, true, true, false,
            false, true, false, true, false, false, true, true, true, false, false, true, true,
            true, true, true, false, false, false, false, false, false, false, false, true, false,
            false, true, true, true, true, false, false, false, false, false, false, true, false,
            false, true,
        ]);
        let image = Image {
            pixels: vec![
                vec![true, false, false, true, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, true, false, false],
                vec![false, false, true, true, true],
            ],
            height: 5,
            width: 5,
            default_pixel: false,
        };

        let expected1 = Image {
            pixels: vec![
                vec![false, true, true, false, true, true, false],
                vec![true, false, false, true, false, true, false],
                vec![true, true, false, true, false, false, true],
                vec![true, true, true, true, false, false, true],
                vec![false, true, false, false, true, true, false],
                vec![false, false, true, true, false, false, true],
                vec![false, false, false, true, false, true, false],
            ],
            width: 7,
            height: 7,
            default_pixel: false,
        };

        let image1 = enhancer.enhance(image);
        assert_eq!(7, image1.width);
        assert_eq!(7, image1.height);
        assert_eq!(expected1.paint(), image1.paint());

        let expected2 = Image {
            pixels: vec![
                vec![false, false, false, false, false, false, false, true, false],
                vec![false, true, false, false, true, false, true, false, false],
                vec![true, false, true, false, false, false, true, true, true],
                vec![true, false, false, false, true, true, false, true, false],
                vec![true, false, false, false, false, false, true, false, true],
                vec![false, true, false, true, true, true, true, true, false],
                vec![false, false, true, false, true, true, true, true, true],
                vec![false, false, false, true, true, false, true, true, false],
                vec![false, false, false, false, true, true, true, false, false],
            ],
            width: 9,
            height: 9,
            default_pixel: false,
        };

        let image2 = enhancer.enhance(image1);
        assert_eq!(9, image2.width);
        assert_eq!(9, image2.height);
        assert_eq!(expected2.paint(), image2.paint());

        assert_eq!(35, expected2.lit_pixels());
    }
}
