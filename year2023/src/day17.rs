use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).context("Invalid digit: {b}"))
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn left(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
    fn right(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Clone, Copy)]
struct Crucible {
    x: usize,
    y: usize,
    dir: Direction,
    steps_left: u8,
}

impl Crucible {
    fn new(dir: Direction, x: usize, y: usize, steps_left: u8) -> Self {
        Self {
            dir,
            x,
            y,
            steps_left,
        }
    }
}

type CrucibleMap = std::collections::HashMap<Crucible, u32>;

fn possible_paths(map: &CrucibleMap, crucible: Crucible) -> Vec<(Crucible, heat_lost)> {
    let x = crucible.x as i32;
    let y = crucible.y as i32;
    let (nx, ny) = match crucible.dir {
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y - 1),
    };
    let mut result = vec![];
    if (0..map.len() as i32).contains(&ny) && (0..map[0].len() as i32).contains(&nx) {
        result.push(Crucible::new(
            crucible.dir.left(),
            nx as usize,
            ny as usize,
            map[y][x][crucible.dir as usize],
            3,
        ));
        result.push(Crucible::new(
            crucible.dir.right(),
            nx as usize,
            ny as usize,
            map[y][x][crucible.dir as usize],
            3,
        ));
        if crucible.steps_left > 1 {
            result.push(Crucible::new(
                crucible.dir,
                nx as usize,
                ny as usize,
                crucible.heat_lost + map[y][x][crucible.dir as usize],
                crucible.steps_left - 1,
            ));
        }
    }
    result
}

fn apply(&mut map: CrucibleMap, path: Crucible) -> bool {
    map.entry(&Crucible).or_insert()
}

pub fn task1(input: &[Vec<u32>]) -> anyhow::Result<u32> {
    let height = input.len();
    anyhow::ensure!(height > 0, "Lava map is empty!");
    let width = input[0].len();
    anyhow::ensure!(width > 0, "Lava map is empty!");
    let mut map = vec![vec![[0; 4]; width]; height];
    let mut paths = vec![];
    paths.extend(&possible_paths(&map, Crucible::new(Direction::Right, 0, 0)));
    paths.extend(&possible_paths(&map, Crucible::new(Direction::Down, 0, 0)));
    while !paths.is_empty() {
        let new_paths = vec![];
        while let Some(path) = paths.pop() {
            if apply(&mut map, path) {
                new_paths.extend(&possible_paths(&map, path));
            }
        }
        paths = new_paths;
    }
    map[height - 1][width - 1]
        .iter()
        .filter(|&&v| v > 0)
        .copied()
        .min()
        .context("Cruicible can't reach the target!")
}

pub fn task2(_input: &[Vec<u32>]) -> anyhow::Result<i32> {
    anyhow::bail!("Todo")
}
