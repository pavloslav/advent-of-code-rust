use crate::*;

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl std::str::FromStr for Command {
    type Err = AocError;
    fn from_str(line: &str) -> AocResult<Command> {
        let (instruction, value) = prse::try_parse!(line, "{} {}")?;

        use Command::*;
        Ok(match instruction {
            "forward" => Forward(value),
            "down" => Down(value),
            "up" => Up(value),
            other => return Err(aoc_error!("Unknown instruction '{other}'")),
        })
    }
}

pub fn parse_input(input: &str) -> AocResult<Vec<Command>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(commands: &[Command]) -> AocResult<i32> {
    use Command::*;
    let (x, y) = commands
        .iter()
        .fold((0, 0), |(x, y), command| match command {
            Forward(dx) => (x + dx, y),
            Down(dy) => (x, y + dy),
            Up(dy) => (x, y - dy),
        });
    Ok(x * y)
}

pub fn task2(commands: &[Command]) -> AocResult<i32> {
    use Command::*;
    let (x, y, _) = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), command| match command {
            Forward(v) => (x + v, y + aim * v, aim),
            Down(da) => (x, y, aim + da),
            Up(da) => (x, y, aim - da),
        });
    Ok(x * y)
}
