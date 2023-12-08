const FIRST_ITERATIONS: usize = 40;
const SECOND_ITERATIONS: usize = 400_000;

pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input.trim())
}

fn get_tile(line: &[u8], i: i32) -> u8 {
    if i < 0 {
        b'.'
    } else {
        *line.get(i as usize).unwrap_or(&b'.')
    }
}

fn get_next_line(line: &[u8]) -> Vec<u8> {
    (0..line.len() as i32)
        .map(|i| {
            if get_tile(line, i - 1) == get_tile(line, i + 1) {
                b'.'
            } else {
                b'^'
            }
        })
        .collect()
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    let mut count = 0;
    let mut line: Vec<_> = input.bytes().collect();
    for _ in 0..FIRST_ITERATIONS {
        count += line.iter().filter(|&&c| c == b'.').count();
        line = get_next_line(&line);
    }
    Ok(count)
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    let mut count = 0;
    let mut line: Vec<_> = input.bytes().collect();
    for _ in 0..SECOND_ITERATIONS {
        count += line.iter().filter(|&&c| c == b'.').count();
        line = get_next_line(&line);
    }
    Ok(count)
}

/*pub fn task2(input: &str) -> anyhow::Result<usize> {
    let (lambda, mu) = crate::common::floyd_hare_tortoise(
        || input.bytes().collect::<Vec<u8>>(),
        |l| {
            let t = get_next_line(&l);
            l.clear();
            l.extend_from_slice(&t[..])
        },
    );
    todo!();
    let tail = (SECOND_ITERATIONS - mu) % lambda;
    let mut count = 0;
    let mut line: Vec<_> = input.bytes().collect();
    let mut mu_count = 0;
    let mut lambda_count = 0;
    let mut tail_count = 0;
    for i in 0..mu + lambda + tail {
        count += line.iter().filter(|&&c| c == b'.').count();
        if i == mu - 1 {
            mu_count = count;
        } else if i == mu + lambda - 1 {
            lambda_count = count - mu_count;
        } else if i == mu + lambda + tail - 1 {
            tail_count = count - lambda_count - mu_count;
        }
        line = get_next_line(&line);
    }
    Ok(
        mu_count
            + (SECOND_ITERATIONS - mu) / lambda * lambda_count
            + tail_count,
    )
}*/
