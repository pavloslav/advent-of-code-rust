pub fn parse_input(input: &str) -> anyhow::Result<usize> {
    Ok(input.trim().parse()?)
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

pub fn task1(input: &usize) -> anyhow::Result<usize> {
    Ok(search(*input, 10, usize::MAX))
}

pub fn task2(input: &usize) -> anyhow::Result<usize> {
    Ok(search(*input, 11, 50))
}
