pub fn parse_input(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[2..].parse().unwrap()))
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Rope(i32, i32);

impl Rope {
    fn pull(&mut self, dir: char) {
        match dir {
            'L' => self.0 -= 1,
            'R' => self.0 += 1,
            'U' => self.1 += 1,
            'D' => self.1 -= 1,
            _ => unimplemented!(),
        }
    }
    fn step(&mut self, after: Rope) {
        if (self.0 - after.0).abs() > 1 || (self.1 - after.1).abs() > 1 {
            self.0 += num::signum(after.0 - self.0);
            self.1 += num::signum(after.1 - self.1);
        }
    }
}
use std::collections::HashSet;

fn run_rope(path: &[(char, usize)], len: usize) -> usize {
    let mut visited = HashSet::new();
    let mut rope = vec![Rope(0, 0); len];
    //visited.insert(rope[len - 1]);
    for &(dir, num) in path {
        for _ in 0..num {
            rope[0].pull(dir);
            for i in 1..len {
                let after = rope[i - 1];
                rope[i].step(after);
            }
            visited.insert(rope[len - 1]);
        }
    }
    visited.len()
}

pub fn task1(path: &[(char, usize)]) -> usize {
    run_rope(path, 2)
}

pub fn task2(path: &[(char, usize)]) -> usize {
    run_rope(path, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let data = parse_input(input);
        assert_eq!(task1(&data), 13);
    }
}
