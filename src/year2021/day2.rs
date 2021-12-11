pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl Command {
    fn new(line: &str) -> Command
    {
        let mut parts = line.split_whitespace();
        let instruction  = parts.next().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        use Command::*;
        match instruction {
            "forward" => Forward(value),
            "down"    => Down(value),
            "up"      => Up(value),
            _         => panic!("{}", instruction)
        }
    }
}

pub fn parse_input(input: &str)-> Vec<Command> {
    input.lines()
         .map(|line| Command::new(line) )
         .collect()
}

pub fn task1(commands: &[Command]) -> i32
{
    use Command::*;
    let (x, y) = commands.iter()
                       .fold((0,0),|(x,y), command|match command {
                        Forward(dx) => (x+dx, y),
                        Down(dy)    => (x,    y+dy),
                        Up(dy)      => (x,    y-dy)
                       });
    x*y
}

pub fn task2(commands: &[Command]) -> i32
{
    use Command::*;
    let (x, y, _) = commands.iter()
                          .fold((0,0,0),|(x,y,aim), command|match command {
                            Forward(v) => (x+v, y+aim*v, aim),
                            Down(da)   => (x,   y,       aim+da),
                            Up(da)     => (x,   y,       aim-da)
                            });
    x*y
}
