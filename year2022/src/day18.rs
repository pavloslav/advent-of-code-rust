use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<(usize, usize, usize)>> {
    input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{},{},{}")?))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Empty,
    Outer,
    Lava,
}

struct Space {
    cubes: Vec<Vec<Vec<State>>>,
    lava: Vec<(usize, usize, usize)>,
}

impl Space {
    fn from_lava(lava: &[(usize, usize, usize)]) -> Space {
        let (max_x, max_y, max_z) = lava
            .iter()
            .fold((0, 0, 0), |(max_x, max_y, max_z), &(x, y, z)| {
                (max_x.max(x), max_y.max(y), max_z.max(z))
            });
        let lava: Vec<_> = lava
            .iter()
            .map(|&(x, y, z)| (x + 1, y + 1, z + 1))
            .collect();
        //+1 for len, +2 for empty planes around
        let mut cubes = vec![vec![vec![State::Empty; max_z + 3]; max_y + 3]; max_x + 3];
        for &(x, y, z) in &lava {
            cubes[x][y][z] = State::Lava;
        }

        Space { cubes, lava }
    }
    fn get_neighbors(
        &self,
        x: usize,
        y: usize,
        z: usize,
    ) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        const SHIFTS: &[(usize, usize, usize)] = &[(1, 0, 0), (0, 1, 0), (0, 0, 1)];
        SHIFTS
            .iter()
            .filter_map(move |&(dx, dy, dz)| {
                if let (Some(x), Some(y), Some(z)) =
                    (x.checked_sub(dx), y.checked_sub(dy), z.checked_sub(dz))
                {
                    Some((x, y, z))
                } else {
                    None
                }
            })
            .chain(SHIFTS.iter().filter_map(move |&(dx, dy, dz)| {
                if x + dx < self.cubes.len()
                    && y + dy < self.cubes[x].len()
                    && z + dz < self.cubes[x][y].len()
                {
                    Some((x + dx, y + dy, z + dz))
                } else {
                    None
                }
            }))
    }
    fn count_lava_neighbors(&self, state: State) -> usize {
        self.lava
            .iter()
            .map(|&(x, y, z)| {
                self.get_neighbors(x, y, z)
                    .filter(|&(dx, dy, dz)| self.cubes[dx][dy][dz] == state)
                    .count()
            })
            .sum()
    }
    fn floodfill(&mut self) {
        let mut stack = vec![(0, 0, 0)];
        while let Some((x, y, z)) = stack.pop() {
            if self.cubes[x][y][z] == State::Empty {
                self.cubes[x][y][z] = State::Outer;
                stack.extend(self.get_neighbors(x, y, z));
            }
        }
    }
}

pub fn task1(lava: &[(usize, usize, usize)]) -> AocResult<usize> {
    let space = Space::from_lava(lava);
    Ok(space.count_lava_neighbors(State::Empty))
}

pub fn task2(lava: &[(usize, usize, usize)]) -> AocResult<usize> {
    let mut space = Space::from_lava(lava);
    space.floodfill();
    Ok(space.count_lava_neighbors(State::Outer))
}
