pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<&[u8]>>> {
    let mut result = vec![vec![]];
    for line in input.lines() {
        if line.is_empty() {
            result.push(vec![]);
        } else {
            result.last_mut().unwrap().push(line.as_bytes());
        }
    }
    Ok(result)
}

pub fn task1(input: &[Vec<&[u8]>]) -> anyhow::Result<usize> {
    let mut sum = 0;
    for pattern in input {
        let height = pattern.len();
        let width = pattern[0].len();

        let hor_mirror = (1..height)
            .find(|&i| (0..i.min(height - i)).all(|j| pattern[i - j - 1] == pattern[i + j]))
            .unwrap_or(0);
        let vert_mirror = (1..width)
            .find(|&i| {
                (0..i.min(width - i))
                    .all(|j| pattern.iter().all(|line| line[i - j - 1] == line[i + j]))
            })
            .unwrap_or(0);
        sum += 100 * hor_mirror + vert_mirror;
    }
    Ok(sum)
}

pub fn task2(input: &[Vec<&[u8]>]) -> anyhow::Result<usize> {
    let mut sum = 0;
    for pattern in input {
        let height = pattern.len();
        let width = pattern[0].len();

        let hor_mirror = (1..height)
            .find(|&i| {
                let mut wrongs = 0;
                for j in 0..i.min(height - i) {
                    for x in 0..width {
                        if pattern[i - j - 1][x] != pattern[i + j][x] {
                            wrongs += 1;
                            if wrongs > 1 {
                                return false;
                            }
                        }
                    }
                }
                wrongs == 1
            })
            .unwrap_or(0);
        let vert_mirror = (1..width)
            .find(|&i| {
                let mut wrongs = 0;
                for j in 0..i.min(width - i) {
                    for line in pattern {
                        if line[i - j - 1] != line[i + j] {
                            wrongs += 1;
                            if wrongs > 1 {
                                return false;
                            }
                        }
                    }
                }
                wrongs == 1
            })
            .unwrap_or(0);
        sum += 100 * hor_mirror + vert_mirror;
    }
    Ok(sum)
}
