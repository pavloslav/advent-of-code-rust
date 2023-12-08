use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    input
        .trim()
        .chars()
        .map(|c| match c.to_digit(10) {
            Some(d) => Ok(d as i32),
            None => Err(anyhow::anyhow!("Wrong digit: '{c}'")),
        })
        .collect()
}

const PHASES: usize = 100;
const PATTERN: [i32; 4] = [0, 1, 0, -1];

pub fn task1(input: &[i32]) -> anyhow::Result<String> {
    let mut signal = input.to_vec();
    for _ in 0..PHASES {
        signal = (1..=signal.len())
            .map(|i| {
                (signal
                    .iter()
                    .enumerate()
                    .map(|(j, d)| d * PATTERN[(j + 1) / i % PATTERN.len()])
                    .sum::<i32>()
                    % 10)
                    .abs()
            })
            .collect();
    }
    signal
        .iter()
        .take(8)
        .map(|&d| char::from_digit(d as u32, 10).with_context(|| format!("Incorrect digit: {d}")))
        .collect()
}

pub fn task2(_input: &[i32]) -> anyhow::Result<usize> {
    Err(anyhow::anyhow!("Todo"))
}
