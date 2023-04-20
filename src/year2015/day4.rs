use super::Result;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

use super::super::common::Md5Hasher;

fn find_hash(init: &str, zeroes: usize) -> Result<usize> {
    Ok((0..)
        .find(|i| {
            Md5Hasher::new_from_str(init)
                .add_str(&i.to_string())
                .has_zeroes(zeroes)
        })
        .expect("Find can't fail on open range"))
}

pub fn task1(input: &str) -> Result<usize> {
    find_hash(input, 5)
}

pub fn task2(input: &str) -> Result<usize> {
    find_hash(input, 6)
}
