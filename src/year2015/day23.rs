pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

struct Computer {
    regs: [usize; 2],
    ip: usize,
    program: Vec<String>,
}

impl Computer {
    fn new(a: usize, program: &[String]) -> Computer {
        Computer {
            regs: [a, 0],
            ip: 0,
            program: program.to_vec(),
        }
    }
    fn reg(name: &str) -> usize {
        (&name[..1] == "b") as usize
    }
    fn jmp(&mut self, offset: isize) -> bool {
        match self.ip.checked_add_signed(offset) {
            Some(offset) if offset < self.program.len() => {
                self.ip = offset;
                true
            }
            _ => false,
        }
    }

    fn step(&mut self) -> bool {
        let mut instruction = self.program[self.ip].split_whitespace();
        match instruction.next().unwrap() {
            "hlf" => {
                self.regs[Computer::reg(instruction.next().unwrap())] /= 2;
            }
            "tpl" => {
                self.regs[Computer::reg(instruction.next().unwrap())] *= 3;
            }
            "inc" => {
                self.regs[Computer::reg(instruction.next().unwrap())] += 1;
            }
            "jmp" => {
                let offset = instruction.next().unwrap().parse().unwrap();
                return self.jmp(offset);
            }
            "jie" => {
                if self.regs[Computer::reg(instruction.next().unwrap())] % 2
                    == 0
                {
                    let offset = instruction.next().unwrap().parse().unwrap();
                    return self.jmp(offset);
                }
            }
            "jio" => {
                if self.regs[Computer::reg(instruction.next().unwrap())] == 1 {
                    let offset = instruction.next().unwrap().parse().unwrap();
                    return self.jmp(offset);
                }
            }
            &_ => unimplemented!(),
        }
        self.ip += 1;
        self.ip < self.program.len()
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

pub fn task1(input: &[String]) -> usize {
    let mut computer = Computer::new(0, input);
    computer.run();
    computer.regs[1]
}

pub fn task2(input: &[String]) -> usize {
    let mut computer = Computer::new(1, input);
    computer.run();
    computer.regs[1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_computer() {
        let input = "inc a
jio a, +2
tpl a
inc a";
        let mut computer = Computer::new(0, &parse_input(input));
        computer.run();
        assert_eq!(computer.regs, [2, 0]);
    }
}
