use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<String>> {
    Ok(input.lines().map(|l| l.to_owned()).collect())
}

pub fn task1(input: &[String]) -> AocResult<u32> {
    let mut s = 0;
    for l in input {
        let first = l
            .chars()
            .find_map(|d| d.to_digit(10))
            .ok_or(aoc_error!("No digits in line!"))?;
        let last = l
            .chars()
            .rev()
            .find_map(|d| d.to_digit(10))
            .ok_or(aoc_error!("No digits in line!"))?;
        s += 10 * first + last;
    }
    Ok(s)
}

pub fn task2(input: &[String]) -> AocResult<u32> {
    let mut sum = 0;
    for l in input {
        let mut s = l.as_str();
        let mut first = None;
        let mut last = 0;
        while !s.is_empty() {
            let mut digit = s.chars().next().unwrap().to_digit(10);
            if digit.is_none() {
                for (value, number) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .enumerate()
                {
                    if s.starts_with(number) {
                        digit = Some(value as u32 + 1);
                        break;
                    }
                }
            }
            s = &s[1..];

            if first.is_none() {
                first = digit;
            }
            if let Some(d) = digit {
                last = d;
            }
        }
        sum += 10 * first.ok_or(aoc_error!("No digits in line!"))? + last;
    }
    Ok(sum)
}
