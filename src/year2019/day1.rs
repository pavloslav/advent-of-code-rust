use super::aoc::*;

pub fn parse_input(input: &str) -> Result<Vec<i64>> {
    input.lines().map(|line| Ok(line.parse()?)).collect()
}

fn fuel_needed(mass: &i64) -> i64 {
    mass / 3 - 2
}

pub fn task1(data: &[i64]) -> Result<i64> {
    Ok(data.iter().map(fuel_needed).sum())
}

fn fuel_needed_for_fuel(mass: &i64) -> i64 {
    (0..)
        .scan(*mass, |m, _| {
            *m = fuel_needed(m);
            Some(*m).filter(|&m| m > 0)
        })
        .sum()
}

pub fn task2(data: &[i64]) -> Result<i64> {
    Ok(data.iter().map(fuel_needed_for_fuel).sum())
}
