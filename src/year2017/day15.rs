use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            Ok(s.split_whitespace()
                .last()
                .ok_or_else(|| task_error!("Empty input!"))?
                .parse()?)
        })
        .collect()
}

struct Generator {
    value: usize,
    factor: usize,
    filter: usize,
}

impl Generator {
    fn generate(&mut self) -> usize {
        loop {
            self.value = (self.value * self.factor) % 2147483647;
            if self.value % self.filter == 0 {
                return self.value;
            }
        }
    }
}

fn task(input: &[usize], filters: &[usize], limit: usize) -> usize {
    let factors = [16807, 48271];
    let mut generators: Vec<_> = (0..=1)
        .map(|i| Generator {
            value: input[i],
            factor: factors[i],
            filter: filters[i],
        })
        .collect();
    (0..limit)
        .filter(|_| {
            generators[0].generate() & 0xFFFF
                == generators[1].generate() & 0xFFFF
        })
        .count()
}

pub fn task1(input: &[usize]) -> Result<usize> {
    Ok(task(input, &[1, 1], 40_000_000))
}

pub fn task2(input: &[usize]) -> Result<usize> {
    Ok(task(input, &[4, 8], 5_000_000))
}
