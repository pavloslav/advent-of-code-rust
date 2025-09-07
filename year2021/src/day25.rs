#[derive(Clone, Copy, PartialEq)]
enum Cucumber {
    Right,
    Down,
    Empty,
}

impl From<char> for Cucumber {
    fn from(c: char) -> Cucumber {
        match c {
            '>' => Cucumber::Right,
            'v' => Cucumber::Down,
            '.' => Cucumber::Empty,
            _ => panic!(),
        }
    }
}

impl Cucumber {
    fn to_char(self) -> char {
        match self {
            Cucumber::Right => '>',
            Cucumber::Down => 'v',
            Cucumber::Empty => '.',
        }
    }
}

#[derive(Clone)]
pub struct Seafloor {
    cucumbers: Vec<Vec<Cucumber>>,
    turn: usize,
}

impl std::fmt::Debug for Seafloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Turn = {}", self.turn)?;
        for line in &self.cucumbers {
            writeln!(
                f,
                "{}",
                line.iter().map(|c| c.to_char()).collect::<String>()
            )?;
        }

        Ok(())
    }
}

impl Seafloor {
    fn step(&mut self) -> bool {
        let height = self.cucumbers.len();
        let width = self.cucumbers[0].len();
        let mut changed = false;

        for cucumber_kind in [Cucumber::Right, Cucumber::Down] {
            let mut new_field = vec![vec![Cucumber::Empty; width]; height];
            for line_idx in 0..height {
                for col_idx in 0..width {
                    if self.cucumbers[line_idx][col_idx] == cucumber_kind {
                        let (line_tgt, col_tgt) =
                            self.target(cucumber_kind, line_idx, col_idx);
                        if self.cucumbers[line_tgt][col_tgt] == Cucumber::Empty
                        {
                            new_field[line_tgt][col_tgt] = cucumber_kind;
                            changed = true;
                        } else {
                            new_field[line_idx][col_idx] = cucumber_kind;
                        }
                    } else if self.cucumbers[line_idx][col_idx]
                        != Cucumber::Empty
                    {
                        new_field[line_idx][col_idx] =
                            self.cucumbers[line_idx][col_idx];
                    }
                }
            }
            self.cucumbers = new_field;
        }
        self.turn += 1;
        changed
    }

    fn target(
        &self,
        kind: Cucumber,
        line: usize,
        col: usize,
    ) -> (usize, usize) {
        match kind {
            Cucumber::Right => (line, (col + 1) % self.cucumbers[line].len()),
            Cucumber::Down => ((line + 1) % self.cucumbers.len(), col),
            _ => panic!(),
        }
    }
}

pub fn parse_input(input: &str) -> Seafloor {
    Seafloor {
        cucumbers: input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect(),
        turn: 0,
    }
}

pub fn task1(seafloor: &Seafloor) -> usize {
    let mut seafloor = seafloor.clone();
    while seafloor.step() {
        //println!("{:?}", seafloor);
    }
    seafloor.turn
}

pub fn task2(_seafloor: &Seafloor) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1, task2};
    #[test]
    fn test_task1() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let seafloor = parse_input(&input);
        assert_eq!(task1(&seafloor), 58);
    }
}
