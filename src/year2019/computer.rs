#[derive(Clone, PartialEq, Eq)]
pub enum ComputerState {
    Normal,
    Halt,
    Input,
}

use std::collections::VecDeque;

#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<isize>,
    pub ip: usize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    pub state: ComputerState,
}

impl Computer {
    pub fn is_halted(&self) -> bool {
        self.state == ComputerState::Halt
    }
    pub fn is_input_blocked(&self) -> bool {
        self.state == ComputerState::Input && self.input.is_empty()
    }

    pub fn step(&mut self) {
        if self.is_halted() || self.is_input_blocked() {
            return;
        }
        self.state = ComputerState::Normal;
        let opcode = self.memory[self.ip] % 100;
        match opcode {
            1 => {
                let src1 = self.get_value(1);
                let src2 = self.get_value(2);
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt as usize] = src1 + src2;
                self.ip += 4;
            }
            2 => {
                let src1 = self.get_value(1);
                let src2 = self.get_value(2);
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt as usize] = src1 * src2;
                self.ip += 4;
            }
            3 => {
                if self.input.is_empty() {
                    self.state = ComputerState::Input;
                    return;
                }
                let tgt = self.memory[self.ip + 1];
                self.memory[tgt as usize] = self.input.pop_front().unwrap();
                self.ip += 2;
            }
            4 => {
                let src = self.get_value(1);
                self.output.push_back(src);
                self.ip += 2;
            }
            5 => {
                let test = self.get_value(1);
                let tgt = self.get_value(2);
                self.ip = if test != 0 { tgt as usize } else { self.ip + 3 };
            }
            6 => {
                let test = self.get_value(1);
                let tgt = self.get_value(2);
                self.ip = if test == 0 { tgt as usize } else { self.ip + 3 };
            }
            7 => {
                let test1 = self.get_value(1);
                let test2 = self.get_value(2);
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt as usize] = isize::from(test1 < test2);
                self.ip += 4;
            }
            8 => {
                let test1 = self.get_value(1);
                let test2 = self.get_value(2);
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt as usize] = isize::from(test1 == test2);
                self.ip += 4;
            }
            99 => {
                self.state = ComputerState::Halt;
            }
            _ => unimplemented!(),
        }
    }
    pub fn new(code: &[isize]) -> Computer {
        Computer {
            memory: code.into(),
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ComputerState::Normal,
        }
    }

    fn get_value(&self, index: usize) -> isize {
        let mut mode = self.memory[self.ip] / 100;
        for _ in 1..index {
            mode /= 10;
        }
        match mode % 10 {
            0 => self.memory[self.memory[self.ip + index] as usize],
            1 => self.memory[self.ip + index],
            _ => unimplemented!(),
        }
    }
    pub fn run(&mut self) {
        while !self.is_halted() && !self.is_input_blocked() {
            self.step();
        }
    }
    pub fn write(&mut self, value: isize) {
        self.input.push_back(value);
    }
    pub fn read(&mut self) -> Option<isize> {
        self.output.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_memory(before: &[isize], after: &[isize]) {
        let mut comp = Computer::new(before);
        comp.run();
        assert_eq!(comp.memory, after);
    }

    #[test]
    fn test_day2() {
        test_memory(
            &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        test_memory(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        test_memory(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        test_memory(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        test_memory(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    fn test_io(code: &[isize], input: &[isize], output: &[isize], msg: &str) {
        let mut comp = Computer::new(code);
        comp.input.extend(input.iter().copied());
        comp.run();
        assert_eq!(comp.output, output, "{msg}");
    }

    #[test]
    fn test_day5() {
        test_io(&[3, 0, 4, 0, 99], &[42], &[42], "Copy input");
        test_memory(&[1002, 4, 3, 4, 33], &[1002, 4, 3, 4, 99]);
        //equal to 8
        test_io(
            &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[8],
            &[1],
            "Equal to 8 - position",
        );
        test_io(
            &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[42],
            &[0],
            "Equal to 8 - position",
        );

        test_io(
            &[3, 3, 1108, -1, 8, 3, 4, 3, 99],
            &[8],
            &[1],
            "Equal to 8 - immediate",
        );
        test_io(
            &[3, 3, 1108, -1, 8, 3, 4, 3, 99],
            &[42],
            &[0],
            "Equal to 8 - immediate",
        );

        //less then 8
        test_io(
            &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[5],
            &[1],
            "Less than 8 - position",
        );
        test_io(
            &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[42],
            &[0],
            "Less than 8 - position",
        );

        test_io(
            &[3, 3, 1107, -1, 8, 3, 4, 3, 99],
            &[5],
            &[1],
            "Less than 8 - immediate",
        );
        test_io(
            &[3, 3, 1107, -1, 8, 3, 4, 3, 99],
            &[42],
            &[0],
            "Less than 8 - immediate",
        );

        //equal to 0
        test_io(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0],
            &[0],
            "Equal to 0 - position",
        );
        test_io(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[42],
            &[1],
            "Equal to 0 - position",
        );

        test_io(
            &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[0],
            &[0],
            "Equal to 0 - immediate",
        );
        test_io(
            &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[42],
            &[1],
            "Equal to 0 - immediate",
        );

        let larger_example = &[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        test_io(larger_example, &[5], &[999], "Large - less than");
        test_io(larger_example, &[8], &[1000], "Large - equal");
        test_io(larger_example, &[42], &[1001], "Large - greater");
    }
}
