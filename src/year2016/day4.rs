use super::super::common::Error;
use super::super::common::Error::TaskError;
use super::super::common::Result;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Room {
    name: String,
    id: usize,
    check_sum: String,
}

impl std::str::FromStr for Room {
    type Err = Error;
    fn from_str(input: &str) -> Result<Room> {
        static INPUT_REGEX: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| {
                regex::Regex::new(
                    r"^(?P<name>.+)-(?P<id>\d+)\[(?P<check_sum>\w+)\]$",
                )
                .unwrap()
            });
        let cap = INPUT_REGEX.captures(input).ok_or_else(|| {
            TaskError(format!("Can't parse the input '{input}'"))
        })?;
        if let (Some(name), Some(id), Some(check_sum)) =
            (cap.name("name"), cap.name("id"), cap.name("check_sum"))
        {
            Ok(Room {
                name: name.as_str().to_string(),
                id: id.as_str().parse()?,
                check_sum: check_sum.as_str().to_string(),
            })
        } else {
            Err(TaskError("Can't find all fields in input".to_string()))
        }
    }
}

impl Room {
    fn verify_checksum(&self) -> bool {
        let mut dict: HashMap<char, usize> = HashMap::new();
        for c in self.name.chars().filter(|c| *c != '-') {
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
        String::from_iter(calc.iter().take(5).map(|x| *x.0)) == self.check_sum
    }
    fn decrypt(&self) -> String {
        const A_CODE: usize = b'a' as usize;
        self.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                _ => char::from(
                    ((u32::from(c) as usize + self.id - A_CODE) % 26 + A_CODE)
                        as u8,
                ),
            })
            .collect()
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Room>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Room]) -> Result<usize> {
    Ok(input
        .iter()
        .filter_map(|r| Some(r.id).filter(|_| r.verify_checksum()))
        .sum())
}

pub fn task2(input: &[Room]) -> Result<usize> {
    input
        .iter()
        .find_map(|room| {
            Some(room.id).filter(|_| {
                room.verify_checksum()
                    && room.decrypt().contains("northpole object")
            })
        })
        .ok_or_else(|| TaskError("No room found".to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_rooms() {
        let inp = parse_input(
            "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]",
        )
        .unwrap();
        assert_eq!(task1(&inp).unwrap(), 1514);
        let room: Room = "qzmt-zixmtkozy-ivhz-343[xxxxx]".parse().unwrap();
        assert_eq!(room.decrypt(), "very encrypted name");
    }
}
