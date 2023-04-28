use super::aoc::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

use once_cell::sync::Lazy;

fn nice1(line: &str) -> Result<bool> {
    static BAD: Lazy<regex::Regex> =
        Lazy::new(|| regex::Regex::new(r"ab|cd|pq|xy").unwrap());
    static PAIR: Lazy<fancy_regex::Regex> =
        Lazy::new(|| fancy_regex::Regex::new(r"(.)\1").unwrap());
    static VOWELS: Lazy<regex::Regex> =
        Lazy::new(|| regex::Regex::new(r"[aeiou]").unwrap());

    Ok(BAD.find(line).is_none()
        && PAIR.is_match(line)?
        && VOWELS.find_iter(line).count() >= 3)
}

pub fn task1(input: &str) -> Result<usize> {
    input
        .lines()
        .try_fold(0, |acc, line| Ok(if nice1(line)? { acc + 1 } else { acc }))
}

fn nice2(line: &str) -> Result<bool> {
    static PAIRS: Lazy<fancy_regex::Regex> =
        Lazy::new(|| fancy_regex::Regex::new(r"(..).*\1").unwrap());
    static MIDDLE: Lazy<fancy_regex::Regex> =
        Lazy::new(|| fancy_regex::Regex::new(r"(.).\1").unwrap());

    Ok(PAIRS.is_match(line)? && MIDDLE.is_match(line)?)
}

pub fn task2(input: &str) -> Result<usize> {
    input
        .lines()
        .try_fold(0, |acc, line| Ok(if nice2(line)? { acc + 1 } else { acc }))
}
