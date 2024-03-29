pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

enum State {
    Normal,
    Garbage,
    Escape,
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    let mut state = State::Normal;
    let mut level = 0;
    let mut sum = 0;
    for c in input.chars() {
        match state {
            State::Normal => match c {
                '{' => {
                    level += 1;
                    sum += level;
                }
                '}' => level -= 1,
                ',' => (),
                '<' => {
                    state = State::Garbage;
                }
                _ => (),
            },
            State::Garbage => match c {
                '>' => state = State::Normal,
                '!' => state = State::Escape,
                _ => (),
            },
            State::Escape => state = State::Garbage,
        }
    }
    Ok(sum)
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    let mut state = State::Normal;
    let mut garbage = 0;
    for c in input.chars() {
        match state {
            State::Normal => {
                if c == '<' {
                    state = State::Garbage
                }
            }
            State::Garbage => match c {
                '>' => state = State::Normal,
                '!' => state = State::Escape,
                _ => garbage += 1,
            },
            State::Escape => state = State::Garbage,
        }
    }
    Ok(garbage)
}
