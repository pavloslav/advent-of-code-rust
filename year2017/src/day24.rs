use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<bool>>> {
    let input: Vec<(usize, usize)> = input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{}/{}")?))
        .collect::<anyhow::Result<_>>()?;
    let size = input
        .iter()
        .map(|(x, y)| x.max(y))
        .max()
        .context("Empty input")?
        + 1;
    let mut map = vec![vec![false; size]; size];
    for (a, b) in input {
        map[a][b] = true;
        map[b][a] = true;
    }
    Ok(map)
}

fn find_strongest(set: &mut [Vec<bool>], start: usize) -> usize {
    let mut best = 0;
    for i in 0..set.len() {
        if set[start][i] {
            set[start][i] = false;
            set[i][start] = false;
            best = best.max(find_strongest(set, i) + start + i);
            set[start][i] = true;
            set[i][start] = true;
        }
    }
    best
}

fn find_longest_strongest(set: &mut [Vec<bool>], start: usize) -> (usize, usize) {
    let mut best = (0, 0);
    for i in 0..set.len() {
        if set[start][i] {
            set[start][i] = false;
            set[i][start] = false;
            let rest = find_longest_strongest(set, i);
            best = best.max((rest.0 + 1, rest.1 + start + i));
            set[start][i] = true;
            set[i][start] = true;
        }
    }
    best
}

pub fn task1(map: &[Vec<bool>]) -> anyhow::Result<usize> {
    let mut map = map.to_vec();
    Ok(find_strongest(&mut map, 0))
}

pub fn task2(map: &[Vec<bool>]) -> anyhow::Result<usize> {
    let mut map = map.to_vec();
    Ok(find_longest_strongest(&mut map, 0).1)
}
