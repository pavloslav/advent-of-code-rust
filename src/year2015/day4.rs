use crate::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

fn find_hash(init: &str, zeroes: usize) -> Result<usize> {
    (0..)
        .find(|i| {
            aoc_common::Md5Hasher::new_from_str(init)
                .add_str(&i.to_string())
                .has_zeroes(zeroes)
        })
        .ok_or_else(|| aoc_error!("unreachable"))
}

pub fn task1(input: &str) -> Result<usize> {
    find_hash(input, 5)
}

pub fn task2(input: &str) -> Result<usize> {
    find_hash(input, 6)
}
