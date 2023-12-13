use itertools::Itertools;

pub struct MapDigest {
    empty_x: Vec<usize>,
    empty_y: Vec<usize>,
    galaxies: Vec<(usize, usize)>,
}

const EMPTY: u8 = b'.';
const GALAXY: u8 = b'#';

pub fn parse_input(input: &str) -> anyhow::Result<MapDigest> {
    let map: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let empty_x = (0..map[0].len())
        .filter(|&x| map.iter().all(|row| row[x] == EMPTY))
        .collect();
    let empty_y = input
        .lines()
        .enumerate()
        .filter_map(|(y, row)| row.bytes().all(|c| c == EMPTY).then_some(y))
        .collect();
    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter_map(move |(x, c)| (c == GALAXY).then_some((x, y)))
        })
        .collect();
    Ok(MapDigest {
        empty_x,
        empty_y,
        galaxies,
    })
}

fn task(digest: &MapDigest, multiplier: usize) -> anyhow::Result<usize> {
    Ok(digest
        .galaxies
        .iter()
        .combinations(2)
        .map(|g| {
            g[0].0.abs_diff(g[1].0)
                + g[0].1.abs_diff(g[1].1)
                + (multiplier - 1)
                    * digest
                        .empty_y
                        .binary_search(&g[0].1)
                        .map_or_else(|e| e, |o| o)
                        .abs_diff(
                            digest
                                .empty_y
                                .binary_search(&g[1].1)
                                .map_or_else(|e| e, |o| o),
                        )
                + (multiplier - 1)
                    * digest
                        .empty_x
                        .binary_search(&g[0].0)
                        .map_or_else(|e| e, |o| o)
                        .abs_diff(
                            digest
                                .empty_x
                                .binary_search(&g[1].0)
                                .map_or_else(|e| e, |o| o),
                        )
        })
        .sum())
}

pub fn task1(input: &MapDigest) -> anyhow::Result<usize> {
    task(input, 2)
}

pub fn task2(input: &MapDigest) -> anyhow::Result<usize> {
    task(input, 1_000_000)
}
