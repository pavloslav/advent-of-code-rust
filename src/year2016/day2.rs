use super::super::common::Result;
use super::Error::TaskError;

fn code(lines: &str, map: &[&str], initial: (usize, usize)) -> Result<String> {
    let mut pos = initial;
    let mut result = String::with_capacity(4);
    for line in lines.split('\n') {
        for mov in line.chars() {
            let old_pos = pos;
            match mov {
                'U' => pos.1 -= 1,
                'R' => pos.0 += 1,
                'D' => pos.1 += 1,
                'L' => pos.0 -= 1,
                _ => {
                    return Err(TaskError(format!("Wrong move '{mov}'")));
                }
            }
            if map[pos.1].chars().nth(pos.0) == Some('X') {
                pos = old_pos;
            }
        }
        result.push(map[pos.1].chars().nth(pos.0).unwrap());
    }
    Ok(result)
}

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

#[rustfmt::skip]
const SMALL_MAP: [&str; 5] = [
    "XXXXX", 
    "X123X", 
    "X456X", 
    "X789X", 
    "XXXXX"];

#[rustfmt::skip]
const BIG_MAP: [&str; 7] = [
    "XXXXXXX", 
    "XXX1XXX", 
    "XX234XX", 
    "X56789X", 
    "XXABCXX", 
    "XXXDXXX", 
    "XXXXXXX",
];

pub fn task1(input: &str) -> Result<String> {
    code(input, &SMALL_MAP, (2, 2))
}

pub fn task2(input: &str) -> Result<String> {
    code(input, &BIG_MAP, (1, 3))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_code() {
        assert_eq!(
            code(
                "ULL
RRDDD
LURDL
UUUUD",
                &SMALL_MAP,
                (2, 2)
            )
            .unwrap(),
            "1985"
        );
        assert_eq!(
            code(
                "ULL
RRDDD
LURDL
UUUUD",
                &BIG_MAP,
                (1, 3)
            )
            .unwrap(),
            "5DB3"
        );
    }
}
