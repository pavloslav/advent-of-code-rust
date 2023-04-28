use super::aoc::*;

pub fn parse_input(input: &str) -> Result<Vec<(u8, u8)>> {
    input
        .lines()
        .map(|s| {
            let (a, b) = scan_fmt::scan_fmt!(s, "{/./} {/./}", char, char)?;
            Ok(((a as u8 - b'A'), (b as u8 - b'X')))
        })
        .collect()
}

pub fn task1(input: &[(u8, u8)]) -> Result<u32> {
    Ok(input
        .iter()
        .map(|&(opponent, figure)| {
            (3 + figure - opponent + 1) as u32 % 3 * 3 + figure as u32 + 1
        })
        .sum())
}

pub fn task2(input: &[(u8, u8)]) -> Result<u32> {
    Ok(input
        .iter()
        .map(|&(opponent, outcome)| {
            outcome as u32 * 3 + (3 + opponent + outcome - 1) as u32 % 3 + 1
        })
        .sum())
}
