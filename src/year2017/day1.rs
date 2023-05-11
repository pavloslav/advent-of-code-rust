use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .find(|s| !s.starts_with("//") && !s.is_empty())
        .ok_or_else(|| {
            aoc_error!("There should be non-comment and non-empty line")
        })?
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| aoc_error!("Can't parse digit '{c}'"))
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
