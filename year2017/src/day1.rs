use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .lines()
        .find(|s| !s.starts_with("//") && !s.is_empty())
        .context("There should be non-comment and non-empty line")?
        .chars()
        .map(|c| {
            c.to_digit(10)
                .with_context(|| format!("Can't parse digit '{c}'"))
        })
        .collect()
}

fn solve(input: &[u32], step: usize) -> u32 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| Some(x).filter(|&x| x == input[(i + step) % input.len()]))
        .sum()
}

pub fn task1(input: &[u32]) -> anyhow::Result<u32> {
    Ok(solve(input, 1))
}

pub fn task2(input: &[u32]) -> anyhow::Result<u32> {
    Ok(solve(input, input.len() / 2))
}
