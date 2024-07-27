pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

use std::sync::LazyLock;
static BAD: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"ab|cd|pq|xy").unwrap());
static PAIR: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r"(.)\1").unwrap());
static VOWELS: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[aeiou]").unwrap());
static PAIRS: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r"(..).*\1").unwrap());
static MIDDLE: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r"(.).\1").unwrap());

fn nice1(line: &str) -> anyhow::Result<bool> {
    Ok(BAD.find(line).is_none() && PAIR.is_match(line)? && VOWELS.find_iter(line).count() >= 3)
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    input
        .lines()
        .filter_map(|line| match nice1(line) {
            Ok(true) => Some(Ok(1)),
            Ok(false) => None,
            Err(err) => Some(Err(err)),
        })
        .sum()
}

fn nice2(line: &str) -> anyhow::Result<bool> {
    Ok(PAIRS.is_match(line)? && MIDDLE.is_match(line)?)
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
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
