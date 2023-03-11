pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .find(|s| !s.starts_with("//") && !s.is_empty())
        .expect("There should be non-comment and non-empty line")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn solve(input: &[u32], step: usize) -> u32 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            if x == input[(i + step) % input.len()] {
                Some(x)
            } else {
                None
            }
        })
        .sum()
}

pub fn task1(input: &[u32]) -> u32 {
    solve(input, 1)
}

pub fn task2(input: &[u32]) -> u32 {
    solve(input, input.len() / 2)
}
