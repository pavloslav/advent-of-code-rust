use crate::*;

pub fn parse_input(input: &str) -> AocResult<usize> {
    Ok(input.trim().parse()?)
}

fn is_open(x: usize, y: usize, fav: usize) -> bool {
    (fav + (x + y) * (x + y + 1) + 2 * x).count_ones() & 1 == 0
}

struct LookUp {
    fav: usize,
    open: Vec<Vec<bool>>,
}

use std::collections::HashSet;

impl LookUp {
    fn new(fav: usize) -> LookUp {
        LookUp {
            fav,
            open: vec![vec![]],
        }
    }
    fn is_open(&mut self, x: usize, y: usize) -> bool {
        if self.open.len() <= y {
            self.open.resize(y + 1, vec![]);
        }
        if self.open[y].len() <= x {
            let tail: Vec<_> = (self.open[y].len()..x + 1)
                .map(|px| is_open(px, y, self.fav))
                .collect();
            self.open[y].extend(tail);
        }
        self.open[y][x]
    }

    fn find_way_len(&mut self, from: (usize, usize), to: (usize, usize)) -> AocResult<usize> {
        let mut to_visit = [from].into();
        for step in 0.. {
            let mut to_visit_next = HashSet::new();
            for (x, y) in to_visit {
                if (x, y) == to {
                    return Ok(step);
                }
                for (px, py) in [
                    (x.saturating_sub(1), y),
                    (x, y.saturating_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                ] {
                    if self.is_open(px, py) {
                        self.open[py][px] = false;
                        to_visit_next.insert((px, py));
                    }
                }
            }
            if to_visit_next.is_empty() {
                return Err(aoc_error!("No way!"));
            }
            to_visit = to_visit_next;
        }
        unreachable!()
    }

    fn count_visited(&mut self, steps: usize, start: (usize, usize)) -> AocResult<usize> {
        let mut to_visit = [start].into();
        let mut all_visited: HashSet<(usize, usize)> = HashSet::new();
        for _ in 0..=steps {
            let mut to_visit_next = HashSet::new();
            for (x, y) in to_visit {
                all_visited.insert((x, y));
                for (px, py) in [
                    (x.saturating_sub(1), y),
                    (x, y.saturating_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                ] {
                    if self.is_open(px, py) {
                        self.open[py][px] = false;
                        to_visit_next.insert((px, py));
                    }
                }
            }
            if to_visit_next.is_empty() {
                return Err(aoc_error!("No way!"));
            }
            to_visit = to_visit_next;
        }
        Ok(all_visited.len())
    }
}

pub fn task1(&input: &usize) -> AocResult<usize> {
    LookUp::new(input).find_way_len((1, 1), (31, 39))
}

pub fn task2(&input: &usize) -> AocResult<usize> {
    LookUp::new(input).count_visited(50, (1, 1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(LookUp::new(10).find_way_len((1, 1), (7, 4)).unwrap(), 11)
    }
}
