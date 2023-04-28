use super::aoc::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

fn find_hash(init: &str, zeroes: usize) -> Result<usize> {
    (0..)
        .find(|i| {
            Md5Hasher::new_from_str(init)
                .add_str(&i.to_string())
                .has_zeroes(zeroes)
        })
        .ok_or_else(|| TaskError("unreachable!()".to_string()))
}

pub fn task1(input: &str) -> Result<usize> {
    find_hash(input, 5)
}

pub fn task2(input: &str) -> Result<usize> {
    find_hash(input, 6)
}
