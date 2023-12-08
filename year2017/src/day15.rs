pub fn parse_input(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            let (_, n): (usize, usize) = prse::try_parse!(s, "Generator {} starts with {}")?;
            Ok(n)
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
        .filter(|_| generators[0].generate() & 0xFFFF == generators[1].generate() & 0xFFFF)
        .count()
}

const FIRST_LIMIT: usize = 40_000_000;
const SECOND_LIMIT: usize = 5_000_000;

pub fn task1(input: &[usize]) -> anyhow::Result<usize> {
    Ok(task(input, &[1, 1], FIRST_LIMIT))
}

pub fn task2(input: &[usize]) -> anyhow::Result<usize> {
    Ok(task(input, &[4, 8], SECOND_LIMIT))
}
