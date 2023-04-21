use super::super::common::Error::TaskError;
use super::super::common::Result;

use std::collections::{HashMap, VecDeque};

type Word = isize;

#[derive(Clone)]
pub struct Computer {
    pub memory: HashMap<Word, Word>,
    ip: Word,
    input: VecDeque<Word>,
    output: VecDeque<Word>,
    relative_base: Word,
}

impl Computer {
    const ADD: Word = 1;
    const MUL: Word = 2;
    const IN: Word = 3;
    const OUT: Word = 4;
    const JNZ: Word = 5;
    const JZ: Word = 6;
    const LS: Word = 7;
    const EQ: Word = 8;
    const ARB: Word = 9;
    const HLT: Word = 99;

    pub fn is_halted(&self) -> bool {
        self.memory[&self.ip] == Computer::HLT
    }
    fn is_input_blocked(&self) -> bool {
        self.memory[&self.ip] == Computer::IN && self.input.is_empty()
    }

    fn step(&mut self) -> Result<()> {
        if self.is_halted() || self.is_input_blocked() {
            return Ok(());
        }
        let opcode = self.memory[&self.ip] % 100;
        match opcode {
            Computer::ADD => {
                let src1 = self.get_value(1)?;
                let src2 = self.get_value(2)?;
                let tgt = self.get_target(3)?;
                self.memory.insert(tgt, src1 + src2);
                self.ip += 4;
            }
            Computer::MUL => {
                let src1 = self.get_value(1)?;
                let src2 = self.get_value(2)?;
                let tgt = self.get_target(3)?;
                self.memory.insert(tgt, src1 * src2);
                self.ip += 4;
            }
            Computer::IN => {
                if self.input.is_empty() {
                    return Ok(());
                }
                let tgt = self.get_target(1)?;
                self.memory.insert(
                    tgt,
                    self.input.pop_front().ok_or_else(|| {
                        TaskError("Input is empty!".to_string())
                    })?,
                );
                self.ip += 2;
            }
            Computer::OUT => {
                let src = self.get_value(1)?;
                self.output.push_back(src);
                self.ip += 2;
            }
            Computer::JNZ => {
                let test = self.get_value(1)?;
                let tgt = self.get_value(2)?;
                self.ip = if test != 0 { tgt } else { self.ip + 3 };
            }
            Computer::JZ => {
                let test = self.get_value(1)?;
                let tgt = self.get_value(2)?;
                self.ip = if test == 0 { tgt } else { self.ip + 3 };
            }
            Computer::LS => {
                let test1 = self.get_value(1)?;
                let test2 = self.get_value(2)?;
                let tgt = self.get_target(3)?;
                self.memory.insert(tgt, Word::from(test1 < test2));
                self.ip += 4;
            }
            Computer::EQ => {
                let test1 = self.get_value(1)?;
                let test2 = self.get_value(2)?;
                let tgt = self.get_target(3)?;
                self.memory.insert(tgt, Word::from(test1 == test2));
                self.ip += 4;
            }
            Computer::ARB => {
                let src = self.get_value(1)?;
                self.relative_base += src;
                self.ip += 2;
            }
            Computer::HLT => {}
            opcode => {
                return Err(TaskError(format!("Unknown opcode: {opcode}")))
            }
        }
        Ok(())
    }
    pub fn new(code: &[Word]) -> Computer {
        Computer {
            memory: code
                .iter()
                .enumerate()
                .map(|(u, &i)| (u as Word, i))
                .collect(),
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
        }
    }

    fn get_value(&self, index: Word) -> Result<Word> {
        let mut mode = self.memory[&self.ip] / 100;
        for _ in 1..index {
            mode /= 10;
        }
        Ok(match mode % 10 {
            0 => *self
                .memory
                .get(&self.memory[&(self.ip + index)])
                .unwrap_or(&0),
            1 => self.memory[&(self.ip + index)],
            2 => *self
                .memory
                .get(&(self.relative_base + self.memory[&(self.ip + index)]))
                .unwrap_or(&0),
            other => Err(TaskError(format!(
                "Unknown source mode {other} in instruction {}",
                self.memory[&self.ip]
            )))?,
        })
    }
    fn get_target(&self, index: Word) -> Result<Word> {
        let mut mode = self.memory[&self.ip] / 100;
        for _ in 1..index {
            mode /= 10;
        }
        Ok(match mode % 10 {
            0 => self.memory[&(self.ip + index)],
            2 => self.relative_base + self.memory[&(self.ip + index)],
            other => Err(TaskError(format!(
                "Unknown target mode {other} in instruction {} on ip {}",
                self.memory[&self.ip], self.ip
            )))?,
        })
    }
    pub fn run(&mut self) -> Result<&mut Self> {
        while !self.is_halted() && !self.is_input_blocked() {
            self.step()?;
        }
        Ok(self)
    }
    pub fn write(&mut self, value: Word) {
        self.input.push_back(value);
    }
    pub fn read(&mut self) -> Result<Word> {
        self.output
            .pop_front()
            .ok_or_else(|| TaskError("Output is empty!".to_string()))
    }
    pub fn prepare_code(input: &str) -> Result<Vec<Word>> {
        input.trim().split(',').map(|x| Ok(x.parse()?)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_memory(before: &[Word], after: &[Word]) {
        let mut comp = Computer::new(before);
        comp.run().unwrap();
        assert_eq!(comp.memory.len(), after.len(), "Memory sizes differ");
        assert!((0..after.len()).all(|i| comp.memory[&(i as Word)] == after[i]));
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

    fn test_io(code: &[Word], input: &[Word], output: &[Word], msg: &str) {
        let mut comp = Computer::new(code);
        comp.input.extend(input.iter().copied());
        comp.run().unwrap();
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

    #[test]
    fn test_day9() {
        let quine = &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101,
            0, 99,
        ];
        test_io(quine, &[], quine, "Quine");
        test_io(
            &[1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            &[],
            &[1219070632396864],
            "16-digit number",
        );
        test_io(
            &[104, 1125899906842624, 99],
            &[],
            &[1125899906842624],
            "Immediate output",
        );
    }
}
