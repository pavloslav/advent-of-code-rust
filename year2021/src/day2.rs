use crate::*;

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl std::str::FromStr for Command {
    type Err = Error;
    fn from_str(line: &str) -> Result<Command> {
        let (instruction, value) =
            scan_fmt::scan_fmt!(line, "{} {}", String, i32)?;

        use Command::*;
        Ok(match instruction.as_str() {
            "forward" => Forward(value),
            "down" => Down(value),
            "up" => Up(value),
            other => return Err(aoc_error!("Unknown instruction '{other}'")),
        })
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Command>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(commands: &[Command]) -> Result<i32> {
    use Command::*;
    let (x, y) =
        commands
            .iter()
            .fold((0, 0), |(x, y), command| match command {
                Forward(dx) => (x + dx, y),
                Down(dy) => (x, y + dy),
                Up(dy) => (x, y - dy),
            });
    Ok(x * y)
}

pub fn task2(commands: &[Command]) -> Result<i32> {
    use Command::*;
    let (x, y, _) =
        commands
            .iter()
            .fold((0, 0, 0), |(x, y, aim), command| match command {
                Forward(v) => (x + v, y + aim * v, aim),
                Down(da) => (x, y, aim + da),
                Up(da) => (x, y, aim - da),
            });
    Ok(x * y)
}
