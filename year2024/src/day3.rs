use regex::Regex;
use std::sync::LazyLock;

pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap());

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    RE.find_iter(input)
        .map(|s| {
            Ok(match s.as_str() {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => {
                    let (a, b) = prse::try_parse!(s.as_str(), "mul({},{})")?;
                    Instruction::Mul(a, b)
                }
            })
        })
        .collect()
}

pub fn task1(input: &[Instruction]) -> anyhow::Result<i32> {
    Ok(input.iter().fold(0, |acc, x| {
        acc + match x {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        }
    }))
}

pub fn task2(input: &[Instruction]) -> anyhow::Result<i32> {
    Ok(input
        .iter()
        .fold((true, 0), |(enabled, val), x| match x {
            Instruction::Mul(a, b) => (enabled, if enabled { val + a * b } else { val }),
            Instruction::Do => (true, val),
            Instruction::Dont => (false, val),
        })
        .1)
}
