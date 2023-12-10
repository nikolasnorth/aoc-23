use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input/day01.txt";

fn part1() -> Result<u32> {
    let mut total = 0;
    let f = File::open(FILENAME)?;
    let b = BufReader::new(f);
    for line in b.lines().flatten() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first_digit = digits
            .next()
            .ok_or_else(|| anyhow!("expected line with at least one digit, but none found"))?;
        let last_digit = digits.last().unwrap_or(first_digit);
        // multiply first_digit by 10 to combine two digits into a single number, e.g. 7 & 8 => 78
        total += first_digit * 10 + last_digit;
    }
    Ok(total)
}

fn part2() -> Result<u32> {
    let mut total = 0;
    let f = File::open(FILENAME)?;
    let b = BufReader::new(f);
    for line in b.lines().flatten() {
        let first_digit = parse_first_digit(&line)?;
        let last_digit = parse_last_digit(&line).unwrap_or(first_digit);
        // multiply first_digit by 10 to combine two digits into a single number, e.g. 7 & 8 => 78
        total += first_digit * 10 + last_digit;
    }
    Ok(total)
}

/// Tries to convert b to the equivalent u32 base-10 digit.
fn byte_as_digit(b: u8) -> Option<u32> {
    return if b >= b'0' && b <= b'9' {
        Some((b - b'0') as u32)
    } else {
        None
    };
}

/// Returns the first occurrence of a base-10 digit (e.g. '2') or a spelled out base-10 digit (e.g. 'one').
fn parse_first_digit(s: &str) -> Result<u32> {
    for (i, c) in s.bytes().enumerate() {
        if let Some(digit) = byte_as_digit(c) {
            return Ok(digit);
        }

        let s = &s[i..];
        if s.starts_with("one") {
            return Ok(1);
        } else if s.starts_with("two") {
            return Ok(2);
        } else if s.starts_with("three") {
            return Ok(3);
        } else if s.starts_with("four") {
            return Ok(4);
        } else if s.starts_with("five") {
            return Ok(5);
        } else if s.starts_with("six") {
            return Ok(6);
        } else if s.starts_with("seven") {
            return Ok(7);
        } else if s.starts_with("eight") {
            return Ok(8);
        } else if s.starts_with("nine") {
            return Ok(9);
        }
    }

    Err(anyhow!("no base-10 digits found"))
}

/// Returns the last occurrence of a base-10 digit (e.g. '2') or a spelled out base-10 digit (e.g. 'one').
fn parse_last_digit(s: &str) -> Result<u32> {
    for (i, c) in s.bytes().rev().enumerate() {
        // also count down from the end
        let i = s.len() - i;

        if let Some(digit) = byte_as_digit(c) {
            return Ok(digit);
        }

        let s = &s[..i];
        if s.ends_with("one") {
            return Ok(1);
        } else if s.ends_with("two") {
            return Ok(2);
        } else if s.ends_with("three") {
            return Ok(3);
        } else if s.ends_with("four") {
            return Ok(4);
        } else if s.ends_with("five") {
            return Ok(5);
        } else if s.ends_with("six") {
            return Ok(6);
        } else if s.ends_with("seven") {
            return Ok(7);
        } else if s.ends_with("eight") {
            return Ok(8);
        } else if s.ends_with("nine") {
            return Ok(9);
        }
    }

    Err(anyhow!("no base-10 digits found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(54561, part1().unwrap());
        assert_eq!(54076, part2().unwrap());
    }
}
