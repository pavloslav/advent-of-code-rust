use crate::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

use once_cell::sync::Lazy;
const BAD: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"ab|cd|pq|xy").unwrap());
const PAIR: Lazy<fancy_regex::Regex> =
    Lazy::new(|| fancy_regex::Regex::new(r"(.)\1").unwrap());
const VOWELS: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"[aeiou]").unwrap());
const PAIRS: Lazy<fancy_regex::Regex> =
    Lazy::new(|| fancy_regex::Regex::new(r"(..).*\1").unwrap());
const MIDDLE: Lazy<fancy_regex::Regex> =
    Lazy::new(|| fancy_regex::Regex::new(r"(.).\1").unwrap());

fn nice1(line: &str) -> Result<bool> {
    Ok(BAD.find(line).is_none()
        && PAIR.is_match(line)?
        && VOWELS.find_iter(line).count() >= 3)
}

pub fn task1(input: &str) -> Result<usize> {
    input
        .lines()
        .filter_map(|line| match nice1(line) {
            Ok(true) => Some(Ok(1)),
            Ok(false) => None,
            Err(err) => Some(Err(err)),
        })
        .sum()
}

fn nice2(line: &str) -> Result<bool> {
    Ok(PAIRS.is_match(line)? && MIDDLE.is_match(line)?)
}

pub fn task2(input: &str) -> Result<usize> {
    input
        .lines()
        .filter_map(|line| match nice2(line) {
            Ok(true) => Some(Ok(1)),
            Ok(false) => None,
            Err(err) => Some(Err(err)),
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_bad() {
        assert!(BAD.find("haegwjzuvuyypxyu").is_some());
        assert!(BAD.find("jchzalrnumimnmhp").is_none());
    }

    #[test]
    fn test_has_pair() {
        assert!(matches!(PAIR.is_match("dvszwmarrgswjxmb"), Ok(true)));
        assert!(matches!(PAIR.is_match("jchzalrnumimnmhp"), Ok(false)));
    }

    #[test]
    fn test_has_vowels() {
        assert!(VOWELS.find_iter("jchzalrnumimnmhp").count() >= 3);
        assert!(VOWELS.find_iter("dvszwmarrgswjxmb").count() < 3);
    }
}
