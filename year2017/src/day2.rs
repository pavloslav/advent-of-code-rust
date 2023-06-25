use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|s| Ok(s.parse()?)).collect())
        .collect()
}

pub fn task1(input: &[Vec<u32>]) -> Result<u32> {
    input
        .iter()
        .map(|line| {
            let max = line
                .iter()
                .max()
                .ok_or_else(|| aoc_error!("Empty data unacceptable!"))?;
            let min = line
                .iter()
                .min()
                .ok_or_else(|| aoc_error!("Empty data unacceptable!"))?;
            Ok(max - min)
        })
        .sum()
}

pub fn task2(input: &[Vec<u32>]) -> Result<u32> {
    Ok(input
        .iter()
        .map(|line| {
            for x in line {
                for y in line {
                    if x != y && x % y == 0 {
                        return x / y;
                    }
                }
            }
            0
        })
        .sum())
}