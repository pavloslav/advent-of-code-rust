use crate::*;

fn code(lines: &str, map: &[&[u8]], initial: (usize, usize)) -> AocResult<String> {
    let mut pos = initial;
    let mut result = String::new();
    for line in lines.split('\n') {
        for mov in line.chars() {
            let old_pos = pos;
            match mov {
                'U' => pos.1 -= 1,
                'R' => pos.0 += 1,
                'D' => pos.1 += 1,
                'L' => pos.0 -= 1,
                _ => {
                    return Err(aoc_error!("Wrong move '{mov}'"));
                }
            }
            if map.get(pos.1).and_then(|line| line.get(pos.0)) == Some(&b'X') {
                pos = old_pos;
            }
        }
        result.push(
            map.get(pos.1)
                .and_then(|line| line.get(pos.0))
                .ok_or_else(|| aoc_error!("Coordinates {}:{} are out of bounds!", pos.0, pos.1))?
                .to_owned()
                .into(),
        );
    }
    Ok(result)
}

pub fn parse_input(input: &str) -> AocResult<&str> {
    Ok(input.trim())
}

#[rustfmt::skip]
const SMALL_MAP: [&[u8]; 5] = [
    b"XXXXX", 
    b"X123X", 
    b"X456X", 
    b"X789X", 
    b"XXXXX"];

#[rustfmt::skip]
const BIG_MAP: [&[u8]; 7] = [
    b"XXXXXXX", 
    b"XXX1XXX", 
    b"XX234XX", 
    b"X56789X", 
    b"XXABCXX", 
    b"XXXDXXX", 
    b"XXXXXXX",
];

pub fn task1(input: &str) -> AocResult<String> {
    code(input, &SMALL_MAP, (2, 2))
}

pub fn task2(input: &str) -> AocResult<String> {
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
