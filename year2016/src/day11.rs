use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Position {
    state: i32,
}

impl Position {
    fn get_item(&self, n: usize) -> i32 {
        self.state >> (n * 2 + 2) & 0x3
    }
    fn set_item(&mut self, n: usize, val: i32) {
        let mask = 0x3 << (n * 2 + 2);
        self.state &= !mask;
        self.state |= (val & 0x3) << (n * 2 + 2);
    }
    fn get_elev(&self) -> i32 {
        self.state & 0x3
    }
    fn set_elev(&mut self, val: i32) {
        self.state &= !0x3;
        self.state |= val & 0x3;
    }

    fn is_done(&self, size: usize) -> bool {
        self.get_elev() == 3
            && (0..2 * size)
                .map(|i| self.get_item(i))
                .all(|item| item == 3)
    }
    fn is_valid(&self, size: usize) -> bool {
        for floor in 0..=3 {
            let mut has_generator = false;
            for i in 0..size {
                if self.get_item(2 * i + 1) == floor {
                    has_generator = true;
                    break;
                }
            }
            if has_generator {
                for i in 0..size {
                    if self.get_item(2 * i) == floor && self.get_item(2 * i + 1) != floor {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn get_elevator_floor(&self, size: usize) -> Vec<usize> {
        let elev = self.get_elev();
        (0..2 * size)
            .filter(|&i| self.get_item(i) == elev)
            .collect()
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<(usize, Position)> {
    let mut position = Position { state: 0 };
    let mut name_map = HashMap::new();
    for line in input.lines() {
        let (floor, list): (String, &str) = prse::try_parse!(line, "The {} floor contains {}.")?;
        if list == "nothing relevant" {
            continue;
        }
        let floor = match floor.as_str() {
            "first" => 0,
            "second" => 1,
            "third" => 2,
            "fourth" => 3,
            other => anyhow::bail!("Unknown floor '{other}'"),
        };
        static SPLIT_REGEX: LazyLock<regex::Regex> =
            LazyLock::new(|| regex::Regex::new(r"(, and )|(, )|( and )").unwrap());
        for item in SPLIT_REGEX.split(list) {
            let name_map_len = name_map.len();
            if let Ok(generator) = prse::try_parse!(item, "a {} generator") {
                let generator: String = generator;
                let idx = *name_map.entry(generator).or_insert(name_map_len);
                position.set_item(2 * idx + 1, floor);
            } else {
                let microchip = prse::try_parse!(item, "a {}-compatible microchip")
                    .map_err(|_| anyhow::anyhow!("List '{list}', item '{item}' fails"))?;
                let idx = *name_map.entry(microchip).or_insert(name_map_len);
                position.set_item(2 * idx, floor);
            }
        }
    }
    Ok((name_map.len(), position))
}

pub fn task(input: &Position, size: usize) -> anyhow::Result<usize> {
    let mut visited = HashSet::new();
    let mut current = HashSet::from([input.clone()]);
    for step in 1.. {
        let mut new = HashSet::new();
        for pos in &current {
            let elevator_floor: Vec<usize> = pos.get_elevator_floor(size);
            for direction in [-1, 1] {
                let new_floor = pos.get_elev() + direction;
                if (0..=3).contains(&new_floor) {
                    for num_items in 1..=2 {
                        for moving in elevator_floor.iter().combinations(num_items) {
                            let mut new_pos = pos.clone();
                            for &item in moving {
                                new_pos.set_item(item, new_floor);
                            }
                            new_pos.set_elev(new_floor);
                            if new_pos.is_done(size) {
                                return Ok(step);
                            }
                            if new_pos.is_valid(size) && !visited.contains(&new_pos) {
                                new.insert(new_pos);
                            }
                        }
                    }
                }
            }
        }
        if new.is_empty() {
            break;
        }
        visited.extend(current);
        current = new;
    }
    Err(anyhow::anyhow!("Solution not found"))
}

pub fn task1((size, position): &(usize, Position)) -> anyhow::Result<usize> {
    task(position, *size)
}

pub fn task2((size, position): &(usize, Position)) -> anyhow::Result<usize> {
    task(position, *size + 2)
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1};

    #[test]
    fn test_task1() {
        let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
        let position = parse_input(input);
        assert_eq!(task1(&position.unwrap()).unwrap(), 11);
    }
}
