use crate::*;

const FIRST_DISK_LEN: usize = 272;
const SECOND_DISK_LEN: usize = 35651584;

pub fn parse_input(input: &str) -> Result<Vec<bool>> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            other => Err(aoc_error!("Unknown byte '{other}'")),
        })
        .collect()
}

pub fn task(input: &[bool], size: usize) -> String {
    let mut data = input.to_vec();
    while data.len() < size {
        let l = data.len();
        data.resize(l * 2 + 1, false);
        for i in 0..l {
            data[2 * l - i] = !data[i];
        }
    }
    data.truncate(size);
    while data.len() % 2 == 0 {
        data = data.chunks(2).map(|pair| pair[0] == pair[1]).collect();
    }
    data.iter().map(|&b| if b { '1' } else { '0' }).collect()
}

pub fn task1(input: &[bool]) -> Result<String> {
    Ok(task(input, FIRST_DISK_LEN))
}

pub fn task2(input: &[bool]) -> Result<String> {
    Ok(task(input, SECOND_DISK_LEN))
}
