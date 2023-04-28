use super::aoc::*;

type Present = (usize, usize, usize);

pub fn parse_input(input: &str) -> Result<Vec<Present>> {
    input
        .lines()
        .map(|line| {
            Ok(scan_fmt::scan_fmt!(line, "{}x{}x{}", usize, usize, usize)?)
        })
        .collect::<Result<_>>()
}

fn wrapping_paper((x, y, z): &(usize, usize, usize)) -> usize {
    let side1 = x * y;
    let side2 = x * z;
    let side3 = y * z;
    2 * (side1 + side2 + side3) + side1.min(side2).min(side3)
}

fn ribbon((x, y, z): &(usize, usize, usize)) -> usize {
    2 * (x + y + z - x.max(y).max(z)) + x * y * z
}

pub fn task1(presents: &[Present]) -> Result<usize> {
    Ok(presents.iter().map(wrapping_paper).sum())
}

pub fn task2(presents: &[Present]) -> Result<usize> {
    Ok(presents.iter().map(ribbon).sum())
}
