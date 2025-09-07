type Order = ((i32, i32), i32);

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Order>> {
    input
        .lines()
        .map(|l| {
            let (dir, value) = prse::try_parse!(l, "{} {}")?;
            let dir = match dir {
                'L' => (-1, 0),
                'R' => (1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                other => {
                    anyhow::bail!("Unknown direction '{other}'");
                }
            };
            Ok((dir, value))
        })
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Rope(i32, i32);

impl Rope {
    fn pull(&mut self, dir: (i32, i32)) {
        self.0 += dir.0;
        self.1 += dir.1;
    }
    fn step(&mut self, after: Rope) {
        if (self.0 - after.0).abs() > 1 || (self.1 - after.1).abs() > 1 {
            self.0 += (after.0 - self.0).signum();
            self.1 += (after.1 - self.1).signum();
        }
    }
}
use std::collections::HashSet;

fn run_rope(path: &[Order], len: usize) -> usize {
    let mut visited = HashSet::new();
    let mut rope = vec![Rope(0, 0); len];
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

pub fn task1(path: &[Order]) -> anyhow::Result<usize> {
    Ok(run_rope(path, 2))
}

pub fn task2(path: &[Order]) -> anyhow::Result<usize> {
    Ok(run_rope(path, 10))
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1};

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
        let data = parse_input(input).unwrap();
        assert_eq!(task1(&data).unwrap(), 13);
    }
}
