pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(PartialEq, Eq, Clone)]
struct Memory {
    blocks: Vec<usize>,
}

impl Memory {
    fn redist(&mut self) {
        let (max_i, max_v) = self
            .blocks
            .iter()
            .enumerate()
            .min_by_key(|&(_, &x)| -(x as isize))
            .map(|(i, &x)| (i, x))
            .unwrap();
        let size = self.blocks.len();
        self.blocks[max_i] = 0;
        let value_to_add = max_v / size;
        let extra = max_v % size;
        for i in 0..extra {
            self.blocks[(max_i + i + 1) % size] += value_to_add + 1;
        }
        for i in extra..self.blocks.len() {
            self.blocks[(max_i + i + 1) % size] += value_to_add;
        }
    }
}

impl From<&[usize]> for Memory {
    fn from(src: &[usize]) -> Memory {
        Memory {
            blocks: src.to_vec(),
        }
    }
}

pub fn floyd(input: &[usize]) -> (usize, usize) {
    let mut hare = Memory::from(input);
    let mut tortoise = Memory::from(input);
    loop {
        hare.redist();
        hare.redist();
        tortoise.redist();
        if hare == tortoise {
            break;
        }
    }
    let mut mu = 0;
    let mut tortoise = Memory::from(input);
    while tortoise != hare {
        hare.redist();
        tortoise.redist();
        mu += 1;
    }
    let mut lam = 1;
    let mut hare = tortoise.clone();
    loop {
        hare.redist();
        if hare == tortoise {
            break;
        }
        lam += 1;
    }
    (lam, mu)
}

pub fn task1(input: &[usize]) -> usize {
    let (lam, mu) = floyd(input);
    mu + lam
}

pub fn task2(input: &[usize]) -> usize {
    floyd(input).0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&[0, 2, 7, 0]), 5);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&[0, 2, 7, 0]), 4);
    }
}
