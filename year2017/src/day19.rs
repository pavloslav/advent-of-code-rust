use anyhow::Context;

pub enum Direction {
    Down,
    Left,
    Up,
    Right,
}

pub struct Packet {
    x: usize,
    y: usize,
    dir: Direction,
    letters: String,
    distance: usize,
}

impl Packet {
    fn step(&mut self, field: &[Vec<u8>]) -> bool {
        let result = match self.dir {
            Direction::Down if self.y + 1 < field.len() && field[self.y + 1][self.x] != b' ' => {
                self.y += 1;
                true
            }
            Direction::Left if self.x > 0 && field[self.y][self.x - 1] != b' ' => {
                self.x -= 1;
                true
            }
            Direction::Up if self.y > 0 && field[self.y - 1][self.x] != b' ' => {
                self.y -= 1;
                true
            }
            Direction::Right
                if self.x + 1 < field[self.y].len() && field[self.y][self.x + 1] != b' ' =>
            {
                self.x += 1;
                true
            }
            _ => false,
        };
        if result {
            self.distance += 1;
            let cell = field[self.y][self.x];
            if cell.is_ascii_alphabetic() {
                self.letters.push(cell as char);
            }
        }
        result
    }
    fn turn(&mut self, field: &[Vec<u8>]) -> bool {
        match self.dir {
            Direction::Down | Direction::Up => {
                self.dir = Direction::Left;
                if !self.step(field) {
                    self.dir = Direction::Right;
                }
            }
            Direction::Left | Direction::Right => {
                self.dir = Direction::Up;
                if !self.step(field) {
                    self.dir = Direction::Down;
                }
            }
        }
        self.step(field)
    }

    fn new(field: &[Vec<u8>]) -> anyhow::Result<Packet> {
        Ok(Packet {
            x: field[0]
                .iter()
                .enumerate()
                .find(|&(_, &b)| b == b'|')
                .context("First line should contain '|'!")?
                .0,
            y: 0,
            dir: Direction::Down,
            letters: String::new(),
            distance: 1,
        })
    }
    fn travel(field: &[Vec<u8>]) -> anyhow::Result<Packet> {
        let mut packet = Packet::new(field)?;
        while packet.step(field) || packet.turn(field) {}
        Ok(packet)
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Packet> {
    Packet::travel(
        &input
            .lines()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<_>>(),
    )
}

pub fn task1(input: &Packet) -> anyhow::Result<String> {
    Ok(input.letters.clone())
}

pub fn task2(input: &Packet) -> anyhow::Result<usize> {
    Ok(input.distance)
}

#[cfg(test)]
mod test {
    use super::*;
    const FIELD: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(FIELD).unwrap()).unwrap(), "ABCDEF");
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input(FIELD).unwrap()).unwrap(), 38);
    }
}
