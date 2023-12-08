use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(c.to_digit(10)
                        .with_context(|| format!("Wrong digit '{c}'"))?
                        as i32)
                })
                .collect()
        })
        .collect()
}

pub fn task1(trees: &[Vec<i32>]) -> anyhow::Result<usize> {
    let mut count = 0;
    for (y, row) in trees.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            if row[..x].iter().all(|&tree| tree < h)
                || row[x + 1..].iter().all(|&tree| tree < h)
                || trees[..y].iter().all(|row| row[x] < h)
                || trees[y + 1..].iter().all(|row| row[x] < h)
            {
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn task2(trees: &[Vec<i32>]) -> anyhow::Result<usize> {
    let mut best = 0;

    for (y, row) in trees.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            let mut score = 1;

            let mut count = 0;
            let mut max = 0;
            for &tree in row[..x].iter().rev() {
                count += 1;
                if tree > max {
                    max = tree;
                }
                if max >= h {
                    break;
                }
            }
            score *= count;

            let mut count = 0;
            let mut max = 0;
            for &tree in &row[x + 1..] {
                count += 1;
                if tree > max {
                    max = tree;
                }
                if max >= h {
                    break;
                }
            }
            score *= count;

            let mut count = 0;
            let mut max = 0;
            for r in trees[..y].iter().rev() {
                count += 1;
                if r[x] > max {
                    max = r[x];
                }
                if max >= h {
                    break;
                }
            }
            score *= count;

            let mut count = 0;
            let mut max = 0;
            for r in trees[y + 1..].iter() {
                count += 1;
                if r[x] > max {
                    max = r[x];
                }
                if max >= h {
                    break;
                }
            }
            score *= count;

            best = std::cmp::max(best, score);
        }
    }
    Ok(best)
}
