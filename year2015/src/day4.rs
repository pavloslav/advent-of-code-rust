use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input.trim())
}

fn find_hash(init: &str, zeroes: usize) -> anyhow::Result<usize> {
    (0..)
        .find(|i| {
            common::Md5Hasher::new_from_str(init)
                .add_str(&i.to_string())
                .has_zeroes(zeroes)
        })
        .context("unreachable!")
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    find_hash(input, 5)
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    find_hash(input, 6)
}
