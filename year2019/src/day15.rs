use super::computer::Computer;

use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> anyhow::Result<Vec<isize>> {
    Computer::prepare_code(input)
}

const NORTH: isize = 1;
const SOUTH: isize = 2;
const WEST: isize = 3;
const EAST: isize = 4;

const WALL: isize = 0;
const MOVE: isize = 1;
const OXYGEN: isize = 2;

type Position = (isize, isize);

fn neighbor((x, y): Position, dir: isize) -> anyhow::Result<Position> {
    Ok(match dir {
        NORTH => (x, y + 1),
        SOUTH => (x, y - 1),
        WEST => (x + 1, y),
        EAST => (x - 1, y),
        other => anyhow::bail!("Unknown direction: {other}"),
    })
}

struct MapSearcher {
    map: HashMap<Position, Computer>,
    positions: Vec<Position>,
}

enum Answer {
    Oxygen(usize),
    Filled(usize),
}

impl MapSearcher {
    fn new(input: &[isize]) -> MapSearcher {
        MapSearcher {
            positions: vec![(0, 0)],
            map: HashMap::from([((0, 0), Computer::new(input))]),
        }
    }
    fn fill(&mut self) -> anyhow::Result<Answer> {
        //359 - too high
        let mut visited: HashSet<Position> = HashSet::from_iter(self.positions.iter().cloned());
        for step in 1.. {
            let mut new_positions = vec![];
            for &pos in &self.positions {
                for dir in NORTH..=EAST {
                    let new_pos = neighbor(pos, dir)?;
                    if !visited.contains(&new_pos) {
                        visited.insert(new_pos);
                        let mut comp = self.map[&pos].clone();
                        comp.write(dir);
                        comp.run()?;
                        match comp.read()? {
                            WALL => (),
                            MOVE => {
                                new_positions.push(new_pos);
                                self.map.entry(new_pos).or_insert(comp.clone());
                            }
                            OXYGEN => {
                                self.map.entry(new_pos).or_insert(comp.clone());
                                self.positions = vec![new_pos];
                                return Ok(Answer::Oxygen(step));
                            }
                            other => {
                                anyhow::bail!("unknown signal {other} on position {pos:?}")
                            }
                        }
                    }
                }
            }
            if new_positions.is_empty() {
                return Ok(Answer::Filled(step - 1));
            }
            self.positions = new_positions;
        }
        Err(anyhow::anyhow!("Unreachable!"))
    }
}

pub fn task1(input: &[isize]) -> anyhow::Result<usize> {
    let mut searcher = MapSearcher::new(input);
    match searcher.fill()? {
        Answer::Oxygen(step) => Ok(step),
        _ => Err(anyhow::anyhow!("Can't find oxygen!")),
    }
}

pub fn task2(input: &[isize]) -> anyhow::Result<usize> {
    let mut searcher = MapSearcher::new(input);
    if let Answer::Oxygen(_) = searcher.fill()? {
        match searcher.fill()? {
            Answer::Oxygen(_) => Err(anyhow::anyhow!("Second oxygen found!")),
            Answer::Filled(step) => Ok(step),
        }
    } else {
        Err(anyhow::anyhow!("Can't find oxygen!"))
    }
}
