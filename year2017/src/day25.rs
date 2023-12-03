use crate::*;

#[derive(Clone)]
pub struct TuringMachine {
    tape_pos: Vec<u8>,
    tape_neg: Vec<u8>,
    cursor: isize,
    states: Vec<[(u8, isize, usize); 2]>,
    state: usize,
    steps: usize,
}

fn name_to_state(name: char) -> AocResult<usize> {
    if name.is_ascii_uppercase() {
        Ok((name as u32 - b'A' as u32) as usize)
    } else {
        Err(aoc_error!("Incorrect state: 'name'"))
    }
}

fn dir_to_isize(dir: &str) -> AocResult<isize> {
    match dir {
        "left" => Ok(-1),
        "right" => Ok(1),
        other => Err(aoc_error!("Incorrect tape direction: '{other}'")),
    }
}

impl TuringMachine {
    fn read(&self) -> usize {
        *if self.cursor < 0 {
            self.tape_neg
                .get(self.cursor.unsigned_abs() - 1)
                .unwrap_or(&0)
        } else {
            self.tape_pos.get(self.cursor.unsigned_abs()).unwrap_or(&0)
        } as usize
    }

    fn write(&mut self, what: u8) {
        if self.cursor < 0 {
            let idx = self.cursor.unsigned_abs();
            if self.tape_neg.len() < idx {
                self.tape_neg.resize(idx, 0);
            }
            self.tape_neg[idx - 1] = what;
        } else {
            let idx = self.cursor.unsigned_abs();
            if self.tape_pos.len() <= idx {
                self.tape_pos.resize(idx + 1, 0);
            }
            self.tape_pos[idx] = what;
        }
    }

    fn step(&mut self) {
        let command = self.states[self.state][self.read()];
        self.write(command.0);
        self.cursor += command.1;
        self.state = command.2;
    }

    fn checksum(&self) -> usize {
        self.tape_neg
            .iter()
            .chain(self.tape_pos.iter())
            .filter(|&&i| i == 1)
            .count()
    }
}

pub fn parse_input(input: &str) -> AocResult<TuringMachine> {
    let mut state = 0;
    let mut steps = 0;
    let mut states = Vec::new();
    for block in input.split("\n\n") {
        if let Ok((lstate, lsteps)) = prse::try_parse!(
            block,
            "Begin in state {}.
Perform a diagnostic checksum after {} steps.",
            char,
            usize
        ) {
            state = name_to_state(lstate)?;
            steps = lsteps;
        } else if let Ok((_state, write0, dir0, switch0, write1, dir1, switch1)) = prse::try_parse!(
            block,
            "In state {}:
If the current value is 0:
    - Write the value {}.
    - Move one slot to the {}.
    - Continue with state {}.
If the current value is 1:
    - Write the value {}.
    - Move one slot to the {}.
    - Continue with state {}.",
            char,
            u8,
            String,
            char,
            u8,
            String,
            char
        ) {
            states.push([
                (write0, dir_to_isize(&dir0)?, name_to_state(switch0)?),
                (write1, dir_to_isize(&dir1)?, name_to_state(switch1)?),
            ]);
        } else {
            return Err(aoc_error!("Unable to parse {block}"));
        }
    }
    Ok(TuringMachine {
        tape_pos: Vec::new(),
        tape_neg: Vec::new(),
        cursor: 0,
        states,
        state,
        steps,
    })
}

pub fn task1(input: &TuringMachine) -> AocResult<usize> {
    let mut machine = input.clone();
    for _step in 0..machine.steps {
        machine.step();
    }
    Ok(machine.checksum())
}

pub fn task2(_input: &TuringMachine) -> AocResult<&'static str> {
    Ok("Success!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
    If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
    If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
    If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
    If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
        let machine = parse_input(input).unwrap();
        assert_eq!(task1(&machine).unwrap(), 3);
    }
}
