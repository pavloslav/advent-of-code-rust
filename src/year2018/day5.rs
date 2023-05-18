use crate::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

fn reduce(polymer: Vec<u8>) -> usize {
    let mut polymer = polymer;
    loop {
        let mut new_polymer = Vec::with_capacity(polymer.len());
        let mut skip = false;
        for i in 0..polymer.len() - 1 {
            if !skip {
                if polymer[i].to_ascii_lowercase()
                    == polymer[i + 1].to_ascii_lowercase()
                    && polymer[i].is_ascii_lowercase()
                        != polymer[i + 1].is_ascii_lowercase()
                {
                    skip = true;
                } else {
                    new_polymer.push(polymer[i]);
                }
            } else {
                skip = false;
            }
        }
        if !skip {
            new_polymer.push(polymer[polymer.len() - 1]);
        }
        if polymer.len() == new_polymer.len() {
            break;
        }
        polymer = new_polymer;
    }
    polymer.len()
}

pub fn task1(input: &str) -> Result<usize> {
    Ok(reduce(input.as_bytes().to_vec()))
}

pub fn task2(input: &str) -> Result<usize> {
    (b'a'..=b'z')
        .map(|out| {
            reduce(
                input
                    .bytes()
                    .filter(|&b| b.to_ascii_lowercase() != out)
                    .collect(),
            )
        })
        .min()
        .ok_or(aoc_error!("Empty polymer!"))
}
