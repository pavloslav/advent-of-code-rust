use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    Ok(input.lines().map(|line| line.as_bytes().to_vec()).collect())
}

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const GUARD: u8 = b'^';
const OBSTACLE: u8 = b'#';
const VISITED: u8 = b'X';

pub fn task1(input: &[Vec<u8>]) -> anyhow::Result<usize> {
    let mut input = input.to_vec();
    let (mut x, mut y) = input
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, c)| **c == GUARD)
                .and_then(|(x, _)| Some((x as i32, y as i32)))
        })
        .context("No guard found on the map")?;
    let mut dir = 0;

    loop {
        input[y as usize][x as usize] = VISITED;
        if let (Ok(nx), Ok(ny)) = (
            usize::try_from(x + DIRS[dir].0),
            usize::try_from(y + DIRS[dir].1),
        ) {
            if input.get(ny).and_then(|line| line.get(nx).copied()) == Some(OBSTACLE) {
                dir = (dir + 1) % 4;
            } else {
                (x, y) = (nx.try_into()?, ny.try_into()?);
            }
        } else {
            break;
        }
    }
    Ok(input
        .iter()
        .map(|line| line.iter().filter(|&&c| c == b'X').count())
        .sum())
}

pub fn task2(_input: &[Vec<u8>]) -> anyhow::Result<i32> {
    anyhow::bail!("Todo")
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1};

    #[test]
    fn test_task1() {
        let input = "....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...";
        assert_eq!(task1(&parse_input(&input).unwrap()).unwrap(), 41);
    }
}
