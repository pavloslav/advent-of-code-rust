pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    Ok(input.lines().map(|line| line.as_bytes().to_vec()).collect())
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get(input: &[Vec<u8>], x: usize, y: usize, dx: isize, dy: isize) -> Option<u8> {
    let x = x as isize + dx;
    let y = y as isize + dy;
    if 0 <= x && 0 <= y {
        input
            .get(y as usize)
            .and_then(|line| line.get(x as usize).copied())
    } else {
        None
    }
}

fn count_words(input: &[Vec<u8>], x: usize, y: usize, word: &[u8]) -> usize {
    DIRS.iter()
        .filter(|&&(dx, dy)| {
            word.iter()
                .enumerate()
                .all(|(i, &c)| get(input, x, y, dx * i as isize, dy * i as isize) == Some(c))
        })
        .count()
}

pub fn task1(input: &[Vec<u8>]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| (0..line.len()).map(move |x| count_words(input, x, y, b"XMAS")))
        .sum())
}

const DIAGS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

fn is_mas(input: &[Vec<u8>], x: usize, y: usize) -> bool {
    if y == 0 || y + 1 >= input.len() || x == 0 || x + 1 >= input[y].len() || input[y][x] != b'A' {
        false
    } else {
        b"MS".iter().all(|&c| {
            DIAGS
                .iter()
                .filter(|&&(dx, dy)| get(input, x, y, dx, dy) == Some(c))
                .count()
                == 2
        }) && get(input, x, y, -1, -1) != get(input, x, y, 1, 1)
    }
}

pub fn task2(input: &[Vec<u8>]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| (0..line.len()).filter(move |&x| is_mas(input, x, y)))
        .count())
}
