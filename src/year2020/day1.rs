use crate::*;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Result<Vec<i64>> {
    input.lines().map(|line| Ok(line.parse()?)).collect()
}

pub fn task1(data: &[i64]) -> Result<i64> {
    let set: HashSet<_> = data.iter().collect();
    data.iter()
        .find(|&value| set.contains(&(2020 - value)))
        .map(|value| value * (2020 - value))
        .ok_or_else(|| aoc_error!("Not found"))
}

pub fn task2(data: &[i64]) -> Result<i64> {
    let set: HashSet<_> = data.iter().collect();
    for (i, first) in data.iter().enumerate() {
        for second in &data[i + 1..] {
            let third = 2020 - first - second;
            if set.contains(&third) {
                return Ok(first * second * third);
            }
        }
    }
    Err(aoc_error!("Not found"))
}
