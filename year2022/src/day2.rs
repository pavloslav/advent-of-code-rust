pub fn parse_input(input: &str) -> anyhow::Result<Vec<(u8, u8)>> {
    input
        .lines()
        .map(|s| {
            let (a, b): (char, char) = prse::try_parse!(s, "{} {}")?;
            Ok(((a as u8 - b'A'), (b as u8 - b'X')))
        })
        .collect()
}

pub fn task1(input: &[(u8, u8)]) -> anyhow::Result<u32> {
    Ok(input
        .iter()
        .map(|&(opponent, figure)| (3 + figure - opponent + 1) as u32 % 3 * 3 + figure as u32 + 1)
        .sum())
}

pub fn task2(input: &[(u8, u8)]) -> anyhow::Result<u32> {
    Ok(input
        .iter()
        .map(|&(opponent, outcome)| {
            outcome as u32 * 3 + (3 + opponent + outcome - 1) as u32 % 3 + 1
        })
        .sum())
}
