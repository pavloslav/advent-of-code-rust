use super::super::common::Error::TaskError;
use super::super::common::Result;

pub fn parse_input(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .find(|s| !s.starts_with("//") && !s.is_empty())
        .ok_or_else(|| {
            TaskError(
                "There should be non-comment and non-empty line".to_string(),
            )
        })?
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| TaskError(format!("Can't parse digit '{c}'")))
        })
        .collect()
}

fn solve(input: &[u32], step: usize) -> u32 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            Some(x).filter(|&x| x == input[(i + step) % input.len()])
        })
        .sum()
}

pub fn task1(input: &[u32]) -> Result<u32> {
    Ok(solve(input, 1))
}

pub fn task2(input: &[u32]) -> Result<u32> {
    Ok(solve(input, input.len() / 2))
}
