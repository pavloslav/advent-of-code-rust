pub fn parse_input(input: &str) -> &str {
    input.trim()
}

use super::super::common::Md5Hasher;

fn find_hash(init: &str, zeroes: usize) -> usize {
    (0..)
        .find(|i| {
            Md5Hasher::from_str(init)
                .add_str(&i.to_string())
                .has_zeroes(zeroes)
        })
        .unwrap()
}

pub fn task1(input: &str) -> usize {
    find_hash(input, 5)
}

pub fn task2(input: &str) -> usize {
    find_hash(input, 6)
}
