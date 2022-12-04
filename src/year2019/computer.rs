#[derive(Clone)]
pub struct Computer {
    pub memory: Vec<isize>,
    pub ip: usize,
    pub input: std::collections::VecDeque<isize>,
    pub output: Vec<isize>,
}

impl Computer {
    pub fn is_halted(&self) -> bool {
        self.memory[self.ip] == 99
    }
    pub fn step(&mut self) {
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
                let tgt = self.memory[self.ip + 1];
                self.memory[tgt as usize] = self.input.pop_front().unwrap();
                self.ip += 2;
            }
            4 => {
                let src = self.get_value(1);
                self.output.push(src);
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
                self.memory[tgt as usize] = if test1 < test2 { 1 } else { 0 };
                self.ip += 4;
            }
            8 => {
                let test1 = self.get_value(1);
                let test2 = self.get_value(2);
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt as usize] = if test1 == test2 { 1 } else { 0 };
                self.ip += 4;
            }

            99 => (),
            _ => unimplemented!(),
        }
    }
    pub fn new(code: &[isize]) -> Computer {
        Computer {
            memory: code.into(),
            ip: 0,
            input: std::collections::VecDeque::new(),
            output: vec![],
        }
    }
    pub fn new_with_input(code: &[isize], input: &[isize]) -> Computer {
        Computer {
            memory: code.into(),
            ip: 0,
            input: std::collections::VecDeque::from_iter(input.iter().copied()),
            output: vec![],
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
}
