use itertools::Itertools;

fn look_and_say(input: &str) -> String {
    use std::fmt::Write;
    input.trim().chars().chunk_by(|&c| c).into_iter().fold(
        String::new(),
        |mut output, (c, group)| {
            let _ = write!(output, "{}{}", group.count(), c);
            output
        },
    )
}

pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

fn task(input: &str, count: usize) -> usize {
    (0..count)
        .fold(input.to_string(), |acc, _| look_and_say(&acc))
        .len()
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    Ok(task(input, 40))
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    Ok(task(input, 50))
}
