use anyhow::anyhow;

type Map<'a> = (Vec<&'a [u8]>, (usize, usize));

pub fn parse_input<'a>(input: &'a str) -> anyhow::Result<Map<'a>> {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == b'S' {
                return Ok((map, (x, y)));
            }
        }
    }
    Err(anyhow!("No starting point"))
}

pub fn task1((map, (x, y)): &Map) -> anyhow::Result<usize> {
    let mut reached: std::collections::HashSet<_> = [(*x, *y)].into();
    for _step in 0..64 {
        let mut new = std::collections::HashSet::new();
        for (x, y) in reached {
            if y > 0 && map[y - 1][x] != b'#' {
                new.insert((x, y - 1));
            }
            if y + 1 < map.len() && map[y + 1][x] != b'#' {
                new.insert((x, y + 1));
            }
            if x > 0 && map[y][x - 1] != b'#' {
                new.insert((x - 1, y));
            }
            if x + 1 < map[y].len() && map[y][x + 1] != b'#' {
                new.insert((x + 1, y));
            }
        }
        reached = new;
    }
    Ok(reached.len())
}

pub fn task2((_map, (_x, _y)): &Map) -> anyhow::Result<usize> {
    anyhow::bail!("Todo")
}
