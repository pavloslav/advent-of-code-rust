pub struct PasswordPolicy {
    a: usize,
    b: usize,
    symbol: char,
    password: String,
}

impl std::str::FromStr for PasswordPolicy {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<PasswordPolicy> {
        let (a, b, symbol, password) = prse::try_parse!(s, "{}-{} {}: {}")?;
        Ok(PasswordPolicy {
            a,
            b,
            symbol,
            password,
        })
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<PasswordPolicy>> {
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

pub fn task1(data: &[PasswordPolicy]) -> anyhow::Result<usize> {
    Ok(data.iter().filter(|&s| is_valid(s)).count())
}

fn is_valid2(policy: &PasswordPolicy) -> bool {
    (policy.password.chars().nth(policy.a - 1) == Some(policy.symbol))
        != (policy.password.chars().nth(policy.b - 1) == Some(policy.symbol))
}

pub fn task2(data: &[PasswordPolicy]) -> anyhow::Result<usize> {
    Ok(data.iter().filter(|&s| is_valid2(s)).count())
}
