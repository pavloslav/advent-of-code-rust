#[derive(Debug, Clone)]
enum Operation
{
    Jmp(i64),
    Acc(i64),
    Nop(i64),
}

type Program = Vec<Operation>;

struct Computer
{
    instruction: usize,
    accumulator: i64,
    program: Program
}

impl Operation
{
    fn from_str(line:&str) -> Operation
    {
        let mut parts = line.split(' ');
        let operation = parts.next().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        match operation {
            "nop" => Operation::Nop(value),
            "acc" => Operation::Acc(value),
            "jmp" => Operation::Jmp(value),
            _ =>panic!("WTF!!!111")
        }
    }
}

impl Computer
{
    fn with_program(code:&str) -> Computer
    {
        Computer {
            instruction: 0,
            accumulator: 0,
            program: code.lines()
                         .map(|line|Operation::from_str(&line))
                         .collect()
        }
    }
    fn tick(&mut self)
    {
        match self.program[self.instruction] {
            Operation::Jmp(offset) => { 
                self.instruction =
                    if offset.is_negative() {
                        self.instruction.checked_sub(offset.abs() as usize)
                    } else {
                        self.instruction.checked_add(offset as usize)
                    }.unwrap();
            },
            Operation::Acc(increment) => { self.accumulator += increment; self.instruction += 1; },
            Operation::Nop(_) => self.instruction += 1
        }
    }
    fn exited(&self) -> bool
    {
        self.instruction == self.program.len()
    }
    fn not_working(&self)->bool
    {
        self.instruction>=self.program.len()
    }
}

fn task1(s:&str) -> i64
{
    let mut computer = Computer::with_program(s);
    let mut visited = vec![false; computer.program.len()];
    while !visited[computer.instruction] {
        visited[computer.instruction] = true;
        computer.tick();
    }
    computer.accumulator
}

fn task2(s:&str) -> i64
{
    let mut computer = Computer::with_program(s);
    println!("Read program {} instructions", computer.program.len());
    for i in 0..computer.program.len() {
        let mut visited = vec![false; computer.program.len()];
        let save = computer.program[i].clone();
        match computer.program[i] {
            Operation::Jmp(1) => continue,
            Operation::Nop(1) => continue,
            Operation::Jmp(offset) => computer.program[i] = Operation::Nop(offset),
            Operation::Nop(offset) => computer.program[i] = Operation::Jmp(offset),
            _ => continue
        }
        computer.instruction = 0;
        computer.accumulator = 0;
        while !computer.not_working() && !visited[computer.instruction] {
            visited[computer.instruction] = true;
            computer.tick();
        }
        if computer.exited() {
            break;
        }
        println!("Change on {} from {:?} to {:?} is {} with {} visited instructions", i, save,
            computer.program[i], 
            if computer.exited() {"success"} else {"fail"}, 
            visited.iter().map(|&x|if x {1} else {0}).sum::<u32>()
            );
        computer.program[i] = save;
    }
    if computer.exited() {
        computer.accumulator
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
     use crate::task2;
    #[test]
    fn test_task2() {
        let input1 = 
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(task2(input1), 8);
    }
}


fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("8","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}