#[derive(Clone)]
pub struct Computer {
    memory: Vec<usize>,
    ip: usize,
}

impl Computer {
    fn is_halted(&self) -> bool {
        self.memory[self.ip] == 99
    }
    fn step(&mut self) {
        match self.memory[self.ip] {
            1 => {
                let src1 = self.memory[self.ip + 1];
                let src2 = self.memory[self.ip + 2];
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt] = self.memory[src1] + self.memory[src2];
                self.ip += 4;
            }
            2 => {
                let src1 = self.memory[self.ip + 1];
                let src2 = self.memory[self.ip + 2];
                let tgt = self.memory[self.ip + 3];
                self.memory[tgt] = self.memory[src1] * self.memory[src2];
                self.ip += 4;
            }
            _ => (),
        }
    }
    fn new(input: &[usize]) -> Computer {
        Computer {
            memory: input.into(),
            ip: 0,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn task1(input: &[usize]) -> usize {
    let mut computer = Computer::new(input);
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    while !computer.is_halted() {
        computer.step();
    }
    computer.memory[0]
}

pub fn task2(input: &[usize]) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::new(input);
            computer.memory[1] = noun;
            computer.memory[2] = verb;
            while !computer.is_halted() {
                computer.step();
            }
            if computer.memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    usize::MAX
}
