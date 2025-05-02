#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

const UD: u8 = b'|';
const LR: u8 = b'-';
const UL: u8 = b'J';
const UR: u8 = b'L';
const DL: u8 = b'7';
const DR: u8 = b'F';

#[allow(dead_code)]
const EMPTY: u8 = b'.';

const START: u8 = b'S';

fn next_dir(field: u8, from: Dir) -> Dir {
    use Dir::*;
    match (field, from) {
        (UD, Up) | (UL, Right) | (UR, Left) => Up,
        (UD, Down) | (DL, Right) | (DR, Left) => Down,
        (LR, Left) | (UL, Down) | (DL, Up) => Left,
        (LR, Right) | (UR, Down) | (DR, Up) => Right,
        _ => Stop,
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<&[u8]>> {
    Ok(input.lines().map(|line| line.as_bytes()).collect())
}

fn start(pipes: &[&[u8]]) -> anyhow::Result<(i32, i32)> {
    for (y, line) in pipes.iter().enumerate() {
        for (x, &p) in line.iter().enumerate() {
            if p == START {
                return Ok((x as i32, y as i32));
            }
        }
    }
    anyhow::bail!("No starting point found!");
}

fn step(pipes: &[&[u8]], (x, y): (i32, i32), dir: Dir) -> ((i32, i32), Dir) {
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
        ((x, y), next_dir(pipes[y as usize][x as usize], dir))
    }
}

pub fn task1(input: &[&[u8]]) -> anyhow::Result<i32> {
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
                return Ok((count + 1) / 2);
            } else {
                break 'inner;
            }
            count += 1;
        }
    }
    anyhow::bail!("No loop")
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum Field {
    Unknown,
    Loop,
    Left,
    Right,
}

/* All enters are from bottom, next left, next right */
const UD_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Loop, Field::Unknown],
    [Field::Left, Field::Loop, Field::Right],
    [Field::Unknown, Field::Loop, Field::Unknown],
];

const LR_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Left, Field::Unknown],
    [Field::Loop, Field::Loop, Field::Loop],
    [Field::Unknown, Field::Right, Field::Unknown],
];

const UL_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Loop, Field::Unknown],
    [Field::Loop, Field::Loop, Field::Right],
    [Field::Unknown, Field::Right, Field::Unknown],
];
const UR_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Loop, Field::Unknown],
    [Field::Left, Field::Loop, Field::Loop],
    [Field::Unknown, Field::Left, Field::Unknown],
];
const DL_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Right, Field::Unknown],
    [Field::Loop, Field::Loop, Field::Right],
    [Field::Unknown, Field::Loop, Field::Unknown],
];
const DR_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Left, Field::Unknown],
    [Field::Left, Field::Loop, Field::Loop],
    [Field::Unknown, Field::Loop, Field::Unknown],
];
const NONE_FILL: [[Field; 3]; 3] = [
    [Field::Unknown, Field::Unknown, Field::Unknown],
    [Field::Unknown, Field::Unknown, Field::Unknown],
    [Field::Unknown, Field::Unknown, Field::Unknown],
];

fn floodfill(fillmap: &mut [Vec<Field>], to_fill: Vec<(i32, i32)>, filler: Field) -> bool {
    let width = fillmap.len() as i32;
    let height = fillmap.len() as i32;
    let mut to_fill = to_fill;
    while let Some((x, y)) = to_fill.pop() {
        if !(0..width).contains(&x) || !(0..height).contains(&y) {
            return false;
        }
        let (ux, uy) = (x as usize, y as usize);
        if fillmap[uy][ux] != filler && fillmap[uy][ux] != Field::Loop {
            fillmap[uy][ux] = filler;
            to_fill.push((x + 1, y));
            to_fill.push((x - 1, y));
            to_fill.push((x, y + 1));
            to_fill.push((x, y - 1));
        }
    }
    true
}

pub fn task2(input: &[&[u8]]) -> anyhow::Result<usize> {
    let width = input[0].len();
    let height = input.len();
    let mut fillmap = vec![];
    let mut left = vec![];
    let mut right = vec![];
    //fill loop, left/right stacks
    let start = start(input)?;
    'outer: for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        let mut position = start;
        let mut dir = dir;
        fillmap = vec![vec![Field::Unknown; width]; height];
        left = vec![];
        right = vec![];
        'inner: loop {
            let (x, y) = (position.1 as usize, position.0 as usize);
            fillmap[y][x] = Field::Loop;
            let (map_to_fill, inverted) = &match (input[y][x], dir) {
                (UD, Dir::Down) => (UD_FILL, true),
                (UD, Dir::Up) => (UD_FILL, false),
                (LR, Dir::Right) => (LR_FILL, true),
                (LR, Dir::Left) => (LR_FILL, false),
                (UL, Dir::Down) => (UL_FILL, true),
                (UL, Dir::Right) => (UL_FILL, false),
                (UR, Dir::Down) => (UR_FILL, true),
                (UR, Dir::Left) => (UR_FILL, false),
                (DL, Dir::Up) => (DL_FILL, false),
                (DL, Dir::Right) => (DL_FILL, true),
                (DR, Dir::Up) => (DR_FILL, false),
                (DR, Dir::Left) => (DR_FILL, true),
                _ => (NONE_FILL, true),
            };
            if map_to_fill != &NONE_FILL {
                for (dy, fill_line) in map_to_fill.iter().enumerate() {
                    for (dx, fill_symbol) in fill_line.iter().enumerate() {
                        let (nx, ny) = ((x + dx - 1) as i32, (y + dy - 1) as i32);

                        match fill_symbol {
                            Field::Loop => {
                                if (0..width as i32).contains(&nx)
                                    && (0..height as i32).contains(&ny)
                                {
                                    fillmap[ny as usize][nx as usize] = Field::Loop
                                }
                            }
                            Field::Left => {
                                if *inverted {
                                    right.push((nx, ny))
                                } else {
                                    left.push((nx, ny))
                                }
                            }
                            Field::Right => {
                                if *inverted {
                                    left.push((nx, ny))
                                } else {
                                    right.push((nx, ny))
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            let (new_pos, new_dir) = step(input, position, dir);
            if new_dir != Dir::Stop {
                position = new_pos;
                dir = new_dir;
            } else if new_pos == start {
                break 'outer;
            } else {
                left.clear();
                right.clear();
                fillmap.clear();
                break 'inner;
            }
        }
    }
    if fillmap.is_empty() {
        anyhow::bail!("No loop")
    }

    //floodfill left and right, find who's inner

    for (to_fill, filler) in [(left, Field::Left), (right, Field::Right)] {
        if floodfill(&mut fillmap, to_fill, filler) {
            //count inner
            return Ok(fillmap
                .iter()
                .map(|line| line.iter().filter(|&&f| f == filler).count())
                .sum());
        }
    }

    anyhow::bail!("Inner not found!!1111")
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
