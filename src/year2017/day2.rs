pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn task1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap())
        .sum()
}

pub fn task2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|line| {
            for x in line {
                for y in line {
                    if x != y && x % y == 0 {
                        return x / y;
                    }
                }
            }
            0
        })
        .sum()
}
