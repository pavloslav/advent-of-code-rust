pub fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

pub fn task1(input: &[Vec<&str>]) -> usize {
    input
        .iter()
        .filter(|passphrase| {
            std::collections::HashSet::<&&str>::from_iter(passphrase.iter())
                .len()
                == passphrase.len()
        })
        .count()
}

fn sorted_str(&s: &&str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.iter().collect()
}

pub fn task2(input: &[Vec<&str>]) -> usize {
    input
        .iter()
        .filter(|passphrase| {
            std::collections::HashSet::<String>::from_iter(
                passphrase.iter().map(sorted_str),
            )
            .len()
                == passphrase.len()
        })
        .count()
}
