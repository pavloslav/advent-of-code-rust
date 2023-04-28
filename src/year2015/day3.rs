use super::aoc::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

#[derive(Clone, Copy)]
struct Santa {
    x: i32,
    y: i32,
}

impl Santa {
    fn step(&mut self, dir: char) -> Result<()> {
        match dir {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => {
                return Err(TaskError(format!("Wrong symbol '{dir}' in input")))
            }
        }
        Ok(())
    }
    fn to_pair(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn task1(input: &str) -> Result<usize> {
    let mut santa = Santa { x: 0, y: 0 };
    let mut visited = std::collections::HashSet::from([santa.to_pair()]);
    for dir in input.chars() {
        santa.step(dir)?;
        visited.insert(santa.to_pair());
    }
    Ok(visited.len())
}

pub fn task2(input: &str) -> Result<usize> {
    let mut santas = [Santa { x: 0, y: 0 }; 2];
    let mut visited = std::collections::HashSet::from([santas[0].to_pair()]);
    for (i, dir) in input.chars().enumerate() {
        santas[i % 2].step(dir)?;
        visited.insert(santas[i % 2].to_pair());
    }
    Ok(visited.len())
}
