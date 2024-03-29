pub fn parse_input(input: &str) -> anyhow::Result<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            let (_, time, pos): (usize, usize, usize) = prse::try_parse!(
                line,
                "Disc #{} has {} positions; at time=0, it is at position {}."
            )?;
            Ok((time, pos))
        })
        .collect()
}

pub fn task(input: &[(usize, usize)]) -> anyhow::Result<usize> {
    let period = input.iter().map(|(size, _)| size).product();
    for moment in 0..period {
        if input
            .iter()
            .enumerate()
            .all(|(i, (size, start))| (i + start + 1 + moment) % size == 0)
        {
            return Ok(moment);
        }
    }
    Err(anyhow::anyhow!("Solution not found"))
}

pub fn task1(input: &[(usize, usize)]) -> anyhow::Result<usize> {
    task(input)
}

pub fn task2(input: &[(usize, usize)]) -> anyhow::Result<usize> {
    let input: Vec<_> = input
        .iter()
        .chain(std::iter::once(&(11, 0)))
        .copied()
        .collect();
    task(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
        assert_eq!(task1(&parse_input(input).unwrap()).unwrap(), 5);
    }
}
