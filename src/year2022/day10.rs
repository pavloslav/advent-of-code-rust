use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<Option<i32>>> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                Ok(None)
            } else if let Ok(val) = scan_fmt::scan_fmt!(line, "addx {}", i32) {
                Ok(Some(val))
            } else {
                Err(task_error!("Can't parse line '{line}'"))
            }
        })
        .collect()
}

struct Computer<'code> {
    code: &'code [Option<i32>],
    x: i32,
    ip: usize,
    wait: bool,
}

impl<'code> Computer<'code> {
    fn new(code: &'code [Option<i32>]) -> Computer<'code> {
        Computer {
            code,
            x: 1,
            ip: 0,
            wait: false,
        }
    }
    fn clock(&mut self) {
        if self.wait {
            self.wait = false;
            self.x += self.code[self.ip].unwrap();
            self.ip += 1;
        } else {
            match self.code[self.ip] {
                Some(_) => self.wait = true,
                None => self.ip += 1,
            }
        }
    }
}

pub fn task1(input: &[Option<i32>]) -> Result<i32> {
    let mut computer = Computer::new(input);
    let sum = (1..240)
        .filter_map(|cycle| {
            let r = if cycle % 40 == 20 {
                Some(cycle * computer.x)
            } else {
                None
            };
            computer.clock();
            r
        })
        .sum();
    Ok(sum)
}

use itertools::Itertools;

pub fn task2(input: &[Option<i32>]) -> Result<String> {
    let mut computer = Computer::new(input);
    let result = (0..6)
        .map(|_row| {
            (0..40)
                .map(|pixel| {
                    let lit = (computer.x - pixel).abs() <= 1;
                    computer.clock();
                    if lit {
                        '#'
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .join("\n");
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(task1(&parse_input(input).unwrap()).unwrap(), 13140);
    }
}
