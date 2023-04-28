use super::super::common::Error;
use super::aoc::*;

pub struct PasswordPolicy {
    a: usize,
    b: usize,
    symbol: char,
    password: String,
}

impl std::str::FromStr for PasswordPolicy {
    type Err = Error;
    fn from_str(s: &str) -> Result<PasswordPolicy> {
        let (a, b, symbol, password) =
            scan_fmt::scan_fmt!(s, "{}-{} {}: {}", usize, usize, char, String)?;
        Ok(PasswordPolicy {
            a,
            b,
            symbol,
            password,
        })
    }
}

pub fn parse_input(input: &str) -> Result<Vec<PasswordPolicy>> {
    input.lines().map(|line| line.parse()).collect()
}

fn is_valid(policy: &PasswordPolicy) -> bool {
    let count = policy
        .password
        .chars()
        .filter(|&c| c == policy.symbol)
        .count();
    (policy.a..=policy.b).contains(&count)
}

pub fn task1(data: &[PasswordPolicy]) -> Result<usize> {
    Ok(data.iter().filter(|&s| is_valid(s)).count())
}

fn is_valid2(policy: &PasswordPolicy) -> bool {
    (policy.password.chars().nth(policy.a - 1) == Some(policy.symbol))
        != (policy.password.chars().nth(policy.b - 1) == Some(policy.symbol))
}

pub fn task2(data: &[PasswordPolicy]) -> Result<usize> {
    Ok(data.iter().filter(|&s| is_valid2(s)).count())
}
