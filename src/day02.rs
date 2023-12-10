use anyhow::{anyhow, Error, Result};
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const FILENAME: &str = "input/day02.txt";

#[derive(Default)]
struct Colour {
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

#[derive(Default)]
struct Game {
    id: u32,
    samples: Vec<Colour>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::default();
        let mut s = s.split(':');

        game.id = s
            .next()
            .ok_or_else(|| anyhow!("expected game id before ':' but none found"))?
            .split(' ')
            .nth(1)
            .ok_or_else(|| anyhow!("expected id but none found"))?
            .parse::<u32>()?;

        let samples = s
            .next()
            .ok_or_else(|| anyhow!("expected colour samples after ':' but none found"))?
            .split(';');

        for sample in samples {
            let mut c = Colour::default();
            for colour in sample.split(',') {
                let mut colour = colour.trim().split(' ');
                let colour_size = colour
                    .next()
                    .ok_or_else(|| anyhow!("expected colour to have associated number but none found"))?
                    .parse::<u8>()?;
                let colour_name = colour
                    .next()
                    .ok_or_else(|| anyhow!("expected colour to have associated name but none found"))?;
                match colour_name {
                    "red" => c.red = Some(colour_size),
                    "green" => c.green = Some(colour_size),
                    "blue" => c.blue = Some(colour_size),
                    _ => return Err(anyhow!("unexpected colour name found: '{colour_name}'")),
                }
            }
            game.samples.push(c);
        }

        Ok(game)
    }
}

fn part1() -> Result<u32> {
    let mut result = 0;
    let f = File::open(FILENAME)?;
    let b = BufReader::new(f);
    for line in b.lines().flatten() {
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        let game = line.parse::<Game>()?;
        for sample in game.samples {
            max_red = max(max_red, sample.red.unwrap_or(0));
            max_green = max(max_green, sample.green.unwrap_or(0));
            max_blue = max(max_blue, sample.blue.unwrap_or(0));
        }
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            result += game.id;
        }
    }
    Ok(result)
}

fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        assert_eq!(1, part1().unwrap());
        // assert_eq!(1, part2());
    }
}
