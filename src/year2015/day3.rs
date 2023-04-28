use super::aoc::*;

type Move = (i32, i32);

fn parse_move(value: u8) -> Result<Move> {
    Ok(match value {
        b'^' => (0, 1),
        b'v' => (0, -1),
        b'>' => (1, 0),
        b'<' => (-1, 0),
        other => {
            return Err(TaskError(format!("Wrong symbol '{other}' in input")))
        }
    })
}

pub fn parse_input(input: &str) -> Result<Vec<Move>> {
    input.bytes().map(parse_move).collect()
}

#[derive(Clone, Copy)]
struct Santa(i32, i32);

impl Santa {
    fn new() -> Santa {
        Santa(0, 0)
    }
    fn step(&mut self, dir: Move) -> (i32, i32) {
        self.0 += dir.0;
        self.1 += dir.1;
        (self.0, self.1)
    }
    fn to_pair(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

pub fn task1(input: &[Move]) -> Result<usize> {
    let mut santa = Santa::new();
    let mut visited = std::collections::HashSet::from([santa.to_pair()]);
    for &dir in input {
        visited.insert(santa.step(dir));
    }
    Ok(visited.len())
}

pub fn task2(input: &[Move]) -> Result<usize> {
    let mut santas = [Santa::new(); 2];
    let mut visited = std::collections::HashSet::from([santas[0].to_pair()]);
    for (i, &dir) in input.iter().enumerate() {
        visited.insert(santas[i % 2].step(dir));
    }
    Ok(visited.len())
}
