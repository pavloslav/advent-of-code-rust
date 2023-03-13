pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|s| s.split_whitespace().last().unwrap().parse().unwrap())
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

pub fn task1(input: &[usize]) -> usize {
    let mut gen_a = Generator {
        value: input[0],
        factor: 16807,
        filter: 1,
    };
    let mut gen_b = Generator {
        value: input[1],
        factor: 48271,
        filter: 1,
    };
    (0..40_000_000)
        .filter(|_| gen_a.generate() & 0xFFFF == gen_b.generate() & 0xFFFF)
        .count()
}

pub fn task2(input: &[usize]) -> usize {
    let mut gen_a = Generator {
        value: input[0],
        factor: 16807,
        filter: 4,
    };
    let mut gen_b = Generator {
        value: input[1],
        factor: 48271,
        filter: 8,
    };
    (0..5_000_000)
        .filter(|_| gen_a.generate() & 0xFFFF == gen_b.generate() & 0xFFFF)
        .count()
}
