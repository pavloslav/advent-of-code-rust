use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<Vec<usize>>> {
    let mut dist = Vec::new();
    let mut name_map = std::collections::HashMap::new();
    for line in input.lines() {
        let (from, to, distance) =
            scan_fmt::scan_fmt!(line, "{} to {} = {}", String, String, usize)?;
        let size = name_map.len();
        let from = *name_map.entry(from).or_insert(size);
        let size = name_map.len();
        let to = *name_map.entry(to).or_insert(size);
        let size = name_map.len();
        if dist.len() < size {
            dist.resize(size, Vec::new());
        }
        if dist[from].len() < size {
            dist[from].resize(size, 0);
        }
        dist[from][to] = distance;
        if dist[to].len() <= size {
            dist[to].resize(size, 0);
        }
        dist[to][from] = distance;
    }
    Ok(dist)
}

use itertools::Itertools; //permutations

fn task(distances: &[Vec<usize>]) -> impl Iterator<Item = usize> + '_ {
    (0..distances.len())
        .permutations(distances.len())
        .map(|comb| {
            comb.windows(2)
                .map(|pair| {
                    if let &[i, j] = pair {
                        distances[i][j]
                    } else {
                        unreachable!()
                    }
                })
                .sum()
        })
}

pub fn task1(distances: &[Vec<usize>]) -> Result<usize> {
    task(distances)
        .min()
        .ok_or_else(|| task_error!("No distances provided!"))
}

pub fn task2(distances: &[Vec<usize>]) -> Result<usize> {
    task(distances)
        .max()
        .ok_or_else(|| task_error!("No distances provided!"))
}
