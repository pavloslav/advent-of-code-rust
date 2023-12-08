use anyhow::Context;

type Cave = Vec<Vec<usize>>;

pub fn parse_input(input: &str) -> anyhow::Result<Cave> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|risk| {
                    Ok(risk
                        .to_digit(10)
                        .with_context(|| format!("Wrong digit {risk}"))?
                        as usize)
                })
                .collect()
        })
        .collect()
}

pub fn dijkstra(cave: &Cave) -> usize {
    use std::collections::BTreeMap;
    let size = cave.len();
    let mut len_cave = vec![vec![None::<usize>; size]; size];
    len_cave[0][0] = Some(0);
    let mut to_search: BTreeMap<usize, Vec<(usize, usize)>> = [(0, [(0, 0)].into())].into();

    while let Some(mut e) = to_search.first_entry() {
        let (key, x, y) = if let Some((x, y)) = e.get_mut().pop() {
            (*e.key(), x, y)
        } else {
            e.remove();
            continue;
        };
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                (Some(x), Some(y)) if x < size && y < size => {
                    let old = len_cave[x][y];
                    len_cave[x][y] = Some(match len_cave[x][y] {
                        Some(risk) => std::cmp::min(risk, key + cave[x][y]),
                        None => key + cave[x][y],
                    });
                    if len_cave[x][y] != old {
                        to_search
                            .entry(len_cave[x][y].unwrap())
                            .or_default()
                            .push((x, y));
                    }
                }
                _ => {}
            }
        }
    }
    len_cave[size - 1][size - 1].unwrap() - len_cave[0][0].unwrap()
}

pub fn task1(cave: &Cave) -> anyhow::Result<usize> {
    Ok(dijkstra(cave))
}

pub fn task2(cave: &Cave) -> anyhow::Result<usize> {
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
    Ok(dijkstra(&real_cave))
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
        assert_eq!(task1(&parse_input(input).unwrap()).unwrap(), 40);
    }
}
