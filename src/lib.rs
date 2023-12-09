mod day01 {
    use anyhow::{anyhow, Result};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub(crate) fn part1() -> Result<u32> {
        let mut total = 0;
        let f = File::open("input/day01.txt")?;
        let b = BufReader::new(f);
        for line in b.lines().flatten() {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits
                .next()
                .ok_or_else(|| anyhow!("expected line with at least one digit, but none found"))?;
            let last_digit = digits
                .last()
                .unwrap_or(first_digit);
            // multiply first_digit by 10 to combine two digits into a single number, e.g. 7 & 8 => 78
            total += first_digit * 10 + last_digit;
        }
        Ok(total)
    }

    pub(crate) fn part2() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(54561, day01::part1().unwrap());
        // assert_eq!(1, day01::part2());
    }
}
