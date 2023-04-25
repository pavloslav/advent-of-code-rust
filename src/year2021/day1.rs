use super::super::common::Result;

pub fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.lines().map(|line| Ok(line.parse()?)).collect()
}

pub fn task1(depths: &[u32]) -> Result<usize> {
    Ok(depths.windows(2).filter(|&w| w[0] < w[1]).count())
}

pub fn task2(depths: &[u32]) -> Result<usize> {
    Ok(depths.windows(4).filter(|&w| w[0] < w[3]).count())
}
