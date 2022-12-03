use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn parse_input(input: &str) -> &str {
    input.trim()
}

fn real_room_sector_id(input: &str) -> Option<u32> {
    let last_dash = input.rfind('-').unwrap();
    let bracket = input.rfind('[').unwrap();
    let symbols = &input[..last_dash];
    let sector_id = input[last_dash + 1..bracket].parse::<u32>().unwrap();
    let check_sum = &input[bracket + 1..bracket + 6];
    let mut dict: HashMap<char, u32> = HashMap::new();
    for c in symbols.chars().filter(|c| *c != '-') {
        *dict.entry(c).or_insert(0) += 1;
    }
    let mut calc: Vec<_> = dict.iter().collect();
    calc.sort_by(|x, y| {
        if x.1 > y.1 || x.1 == y.1 && x.0 < y.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    if String::from_iter(calc.iter().take(5).map(|x| *x.0)) == check_sum {
        Some(sector_id)
    } else {
        None
    }
}

pub fn task1(input: &str) -> u32 {
    input.lines().filter_map(real_room_sector_id).sum()
}

fn decypher(line: &str, shift: u32) -> String {
    let a_code = b'a' as u32;
    line.chars()
        .map(|c| match c {
            '-' => ' ',
            _ => char::from(
                ((u32::from(c) + shift - a_code) % 26 + a_code) as u8,
            ),
        })
        .collect()
}

pub fn task2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            if let Some(sector_id) = real_room_sector_id(line) {
                if decypher(line, sector_id).contains("northpole object") {
                    return Some(sector_id);
                }
            }
            None
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_rooms() {
        let inp = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
        assert_eq!(task1(&inp), 1514);
        let inp = "qzmt-zixmtkozy-ivhz";
        assert_eq!(decypher(&inp, 343), "very encrypted name");
    }
}
