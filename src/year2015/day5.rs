use super::super::common::Result;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

fn has_bad(line: &str) -> bool {
    const BAD: [&str; 4] = ["ab", "cd", "pq", "xy"];
    (0..line.len() - 1).any(|i| BAD.contains(&&line[i..i + 2]))
}

fn has_pair(line: &str) -> bool {
    (0..line.len() - 1).any(|i| line[i..i + 1] == line[i + 1..i + 2])
}

fn has_vowels(line: &str, count: usize) -> bool {
    line.chars().filter(|&c| "aeiou".contains(c)).count() >= count
}

pub fn task1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .filter(|&line| !has_bad(line) && has_pair(line) && has_vowels(line, 3))
        .count())
}

fn has_two_pairs(line: &str) -> bool {
    (0..line.len() - 3).any(|i| line[i + 2..].contains(&line[i..i + 2]))
}

fn has_middle(line: &str) -> bool {
    (0..line.len() - 2).any(|i| line[i..i + 1] == line[i + 2..i + 3])
}

pub fn task2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .filter(|&line| has_two_pairs(line) && has_middle(line))
        .count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_bad() {
        assert!(has_bad("haegwjzuvuyypxyu"));
        assert!(!has_bad("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_has_pair() {
        assert!(has_pair("dvszwmarrgswjxmb"));
        assert!(!has_pair("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_has_vowels() {
        assert!(has_vowels("jchzalrnumimnmhp", 3));
        assert!(!has_vowels("dvszwmarrgswjxmb", 3));
    }
}
