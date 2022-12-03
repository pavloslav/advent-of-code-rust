type Cave = Vec<Vec<usize>>;

pub fn parse_input(input: &str) -> Cave {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|risk| risk.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn dijkstra(cave: &Cave) -> usize {
    use std::collections::BTreeMap;
    let size = cave.len();
    let mut len_cave = vec![vec![None::<usize>; size]; size];
    len_cave[0][0] = Some(0);
    let mut to_search: BTreeMap<usize, Vec<(usize, usize)>> =
        [(0, [(0, 0)].into())].into();
    while !to_search.is_empty() {
        let min_key = *to_search.keys().next().unwrap();
        let (x, y) = to_search.get_mut(&min_key).unwrap().pop().unwrap();
        if to_search[&min_key].is_empty() {
            to_search.remove(&min_key);
        }
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let x = x as i32 + dx;
            let y = y as i32 + dy;
            if 0 <= x && x < size as i32 && 0 <= y && y < size as i32 {
                let x = x as usize;
                let y = y as usize;
                let old = len_cave[x][y];
                len_cave[x][y] = Some(match len_cave[x][y] {
                    Some(risk) => std::cmp::min(risk, min_key + cave[x][y]),
                    None => min_key + cave[x][y],
                });
                if len_cave[x][y] != old {
                    to_search
                        .entry(len_cave[x][y].unwrap())
                        .or_default()
                        .push((x, y));
                }
            }
        }
    }
    len_cave[size - 1][size - 1].unwrap() - len_cave[0][0].unwrap()
}

pub fn task1(cave: &Cave) -> usize {
    dijkstra(cave)
}

pub fn task2(cave: &Cave) -> usize {
    let size = cave.len();
    let mut real_cave = vec![vec![0; size * 5]; size * 5];
    for i_mult in 0..5 {
        for j_mult in 0..5 {
            for i in 0..size {
                for j in 0..size {
                    real_cave[i_mult * size + i][j_mult * size + j] =
                        (cave[i][j] + i_mult + j_mult - 1) % 9 + 1;
                }
            }
        }
    }
    dijkstra(&real_cave)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(task1(&parse_input(input)), 40);
    }
}
