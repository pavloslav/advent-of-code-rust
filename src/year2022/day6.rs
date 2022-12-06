pub fn parse_input(input: &str) -> &str {
    input
}

use std::collections::HashSet;

fn first_different(input: &str, length: usize) -> usize {
    for i in 0..input.len() - length {
        let set: HashSet<_> = input[i..i + length].chars().collect();
        if set.len() == length {
            return i + length;
        }
    }
    unreachable!()
}

pub fn task1(input: &str) -> usize {
    first_different(input, 4)
}

pub fn task2(input: &str) -> usize {
    first_different(input, 14)
}
