#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Pipe {
    UD,
    LR,
    UL,
    UR,
    DL,
    DR,
    Empty,
    Start,
}

impl Pipe {
    fn next(&self, from: Dir) -> Dir {
        use Dir::*;
        use Pipe::*;
        match (*self, from) {
            (UD, Up) | (UL, Right) | (UR, Left) => Up,
            (UD, Down) | (DL, Right) | (DR, Left) => Down,
            (LR, Left) | (UL, Down) | (DL, Up) => Left,
            (LR, Right) | (UR, Down) | (DR, Up) => Right,
            _ => Stop,
        }
    }
}

impl std::convert::TryFrom<char> for Pipe {
    type Error = anyhow::Error;
    fn try_from(c: char) -> anyhow::Result<Self> {
        use Pipe::*;
        Ok(match c {
            '|' => UD,
            '-' => LR,
            'J' => UL,
            'L' => UR,
            '7' => DL,
            'F' => DR,
            '.' => Empty,
            'S' => Start,
            other => anyhow::bail!("Unknown symbol '{other}'"),
        })
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<Pipe>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(Pipe::try_from)
                .collect::<anyhow::Result<Vec<Pipe>>>()
        })
        .collect()
}

fn start(pipes: &[Vec<Pipe>]) -> anyhow::Result<(i32, i32)> {
    for (y, line) in pipes.iter().enumerate() {
        for (x, &p) in line.iter().enumerate() {
            if p == Pipe::Start {
                return Ok((x as i32, y as i32));
            }
        }
    }
    anyhow::bail!("No starting point found!");
}

fn step(pipes: &[Vec<Pipe>], (x, y): (i32, i32), dir: Dir) -> ((i32, i32), Dir) {
    use Dir::*;
    let (x, y) = match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
        Stop => (x, y),
    };
    if dir == Stop || y < 0 || y >= pipes.len() as i32 || x < 0 || x >= pipes[0].len() as i32 {
        ((x, y), Stop)
    } else {
        ((x, y), pipes[y as usize][x as usize].next(dir))
    }
}

pub fn task1(input: &[Vec<Pipe>]) -> anyhow::Result<i32> {
    let start = start(input)?;
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        let mut count = 0;
        let mut position = start;
        let mut dir = dir;
        'inner: loop {
            let (new_pos, new_dir) = step(input, position, dir);
            if new_dir != Dir::Stop {
                position = new_pos;
                dir = new_dir;
            } else if new_pos == start {
                println!("{count}");
                return Ok((count + 1) / 2);
            } else {
                break 'inner;
            }
            count += 1;
        }
    }
    anyhow::bail!("No loop")
}

pub fn task2(_input: &[Vec<Pipe>]) -> anyhow::Result<i32> {
    anyhow::bail!("Todo")
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 8);
    }
}
