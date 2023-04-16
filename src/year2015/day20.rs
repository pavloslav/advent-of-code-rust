use super::super::common::Result;
use super::Error::TaskError;

pub fn parse_input(input: &str) -> Result<usize> {
    input
        .trim()
        .parse()
        .map_err(|_| TaskError("Wrong input").to_string())
}

const SIEVE_SIZE: usize = 10_000_000;

fn search(num: usize, mult: usize, limit: usize) -> usize {
    let mut sieve = vec![0; SIEVE_SIZE];
    for i in 1..SIEVE_SIZE {
        if (sieve[i] + i) * mult > num {
            return i;
        }
        for j in 1..(SIEVE_SIZE / i).min(limit) {
            sieve[i * j] += i;
        }
    }
    unreachable!()
}

pub fn task1(input: &usize) -> Result<usize> {
    Ok(search(*input, 10, usize::MAX))
}

pub fn task2(input: &usize) -> Result<usize> {
    Ok(search(*input, 11, 50))
}
