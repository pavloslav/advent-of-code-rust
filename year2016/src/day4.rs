use anyhow::Context;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Room {
    name: String,
    id: usize,
    check_sum: String,
}

impl std::str::FromStr for Room {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> anyhow::Result<Room> {
        let (name_id, check_sum): (&str, String) = prse::try_parse!(input, "{}[{}]")?;
        let (name, id) = name_id.split_at(
            name_id
                .find(|c: char| c.is_ascii_digit())
                .context("Can't find digits in room name")?,
        );
        Ok(Room {
            name: name.to_string(),
            id: id.parse()?,
            check_sum,
        })
    }
}

impl Room {
    fn verify_checksum(&self) -> bool {
        let mut dict: HashMap<char, usize> = HashMap::new();
        for c in self.name.chars().filter(|c| *c != '-') {
            dict.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut calc: Vec<_> = dict.iter().collect();
        calc.sort_by_key(|x| (std::cmp::Reverse(x.1), x.0));
        String::from_iter(calc.iter().take(5).map(|x| *x.0)) == self.check_sum
    }
    fn decrypt(&self) -> String {
        const A_CODE: usize = b'a' as usize;
        self.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                _ => char::from(((u32::from(c) as usize + self.id - A_CODE) % 26 + A_CODE) as u8),
            })
            .collect()
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Room>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Room]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .filter_map(|r| Some(r.id).filter(|_| r.verify_checksum()))
        .sum())
}

pub fn task2(input: &[Room]) -> anyhow::Result<usize> {
    input
        .iter()
        .find_map(|room| {
            Some(room.id)
                .filter(|_| room.verify_checksum() && room.decrypt().contains("northpole object"))
        })
        .context("No room found")
}

#[cfg(test)]
mod test {
    use super::{Room, parse_input, task1};

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
        assert_eq!(room.decrypt(), "very encrypted name ");
    }
}
