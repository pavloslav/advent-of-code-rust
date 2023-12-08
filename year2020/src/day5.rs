use anyhow::Context;

fn seat_id(code: &str) -> anyhow::Result<usize> {
    code.chars()
        .map(|c| match c {
            'F' | 'L' => Ok(0),
            'B' | 'R' => Ok(1),
            other => Err(anyhow::anyhow!("Incorrect zone/seat symbol '{other}'")),
        })
        .try_fold(0, |acc, n| Ok(acc * 2 + n?))
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<usize>> {
    input.lines().map(seat_id).collect()
}

pub fn task1(s: &[usize]) -> anyhow::Result<usize> {
    s.iter().max().copied().context("Empty input!")
}

pub fn task2(s: &[usize]) -> anyhow::Result<usize> {
    let (min, max, sum) = s.iter().fold((1024, 0, 0), |(min, max, sum), &id| {
        (min.min(id), max.max(id), sum + id)
    });
    Ok((max - min + 1) * (max + min) / 2 - sum)
}
