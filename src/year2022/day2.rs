pub fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|s| {
            if let (Some(a), Some(b)) = (s.chars().next(), s.chars().nth(2)) {
                ((a as u8 - b'A'), (b as u8 - b'X'))
            } else {
                panic!()
            }
        })
        .collect()
}

pub fn task1(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|&(opponent, figure)| {
            (3 + figure - opponent + 1) as u32 % 3 * 3 + figure as u32 + 1
        })
        .sum()
}

pub fn task2(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|&(opponent, outcome)| {
            outcome as u32 * 3 + (3 + opponent + outcome - 1) as u32 % 3 + 1
        })
        .sum()
}
