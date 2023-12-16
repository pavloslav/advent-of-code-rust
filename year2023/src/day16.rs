use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<&[u8]>> {
    Ok(input.lines().map(str::as_bytes).collect())
}

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

impl Dir {
    fn step(&self, x: usize, y: usize, width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Dir::Up if y > 0 => Some((x, y - 1)),
            Dir::Left if x > 0 => Some((x - 1, y)),
            Dir::Right if x + 1 < width => Some((x + 1, y)),
            Dir::Down if y + 1 < height => Some((x, y + 1)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Tile([bool; 4]);

impl Tile {
    fn put(&mut self, dir: Dir) -> bool {
        let old = self.0[dir as usize];
        self.0[dir as usize] = true;
        old
    }
}

fn energyze(contraption: &[&[u8]], start: (usize, usize, Dir)) -> usize {
    let height = contraption.len();
    let width = contraption[0].len();
    let mut beams = vec![start];
    let mut beams_map = vec![vec![Tile::default(); width]; height];
    while !beams.is_empty() {
        let mut new_beams = vec![];
        while let Some((x, y, dir)) = beams.pop() {
            if beams_map[y][x].put(dir) {
                continue;
            }
            match contraption[y][x] {
                b'/' => {
                    let new_dir = match dir {
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                    };
                    if let Some((nx, ny)) = new_dir.step(x, y, width, height) {
                        new_beams.push((nx, ny, new_dir));
                    }
                }
                b'\\' => {
                    let new_dir = match dir {
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                    };
                    if let Some((nx, ny)) = new_dir.step(x, y, width, height) {
                        new_beams.push((nx, ny, new_dir));
                    }
                }
                b'|' if dir == Dir::Left || dir == Dir::Right => {
                    for new_dir in [Dir::Up, Dir::Down] {
                        if let Some((nx, ny)) = new_dir.step(x, y, width, height) {
                            new_beams.push((nx, ny, new_dir));
                        }
                    }
                }
                b'-' if dir == Dir::Up || dir == Dir::Down => {
                    for new_dir in [Dir::Left, Dir::Right] {
                        if let Some((nx, ny)) = new_dir.step(x, y, width, height) {
                            new_beams.push((nx, ny, new_dir));
                        }
                    }
                }
                _ => {
                    if let Some((nx, ny)) = dir.step(x, y, width, height) {
                        new_beams.push((nx, ny, dir));
                    }
                }
            }
        }
        beams = new_beams;
    }
    beams_map
        .iter()
        .map(|line| line.iter().filter(|tile| tile.0.iter().any(|&x| x)).count())
        .sum()
}

pub fn task1(input: &[&[u8]]) -> anyhow::Result<usize> {
    let height = input.len();
    anyhow::ensure!(height > 0, "Contraption is empty!");
    let width = input[0].len();
    anyhow::ensure!(width > 0, "Contraption is empty!");
    Ok(energyze(input, (0, 0, Dir::Right)))
}

pub fn task2(input: &[&[u8]]) -> anyhow::Result<usize> {
    let height = input.len();
    anyhow::ensure!(height > 0, "Contraption is empty!");
    let width = input[0].len();
    anyhow::ensure!(width > 0, "Contraption is empty!");
    (0..height)
        .map(|y| energyze(input, (0, y, Dir::Right)))
        .chain((0..height).map(|y| energyze(input, (width - 1, y, Dir::Left))))
        .chain((0..width).map(|x| energyze(input, (x, 0, Dir::Down))))
        .chain((0..width).map(|x| energyze(input, (x, height - 1, Dir::Up))))
        .max()
        .context("Something terribly wrong!")
}
