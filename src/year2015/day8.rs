pub fn parse_input(input: &str) -> &str {
    input
}

enum State {
    Normal,
    Slash,
    X1,
    X2,
}

pub fn task1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut count = 0;
            let mut state = State::Normal;

            for ch in line.chars() {
                match state {
                    State::Normal => {
                        if ch == '\\' {
                            state = State::Slash;
                        }
                    }
                    State::Slash => {
                        count += 1;
                        state = if ch == 'x' { State::X1 } else { State::Normal };
                    }
                    State::X1 => {
                        count += 1;
                        state = State::X2;
                    }
                    State::X2 => {
                        count += 1;
                        state = State::Normal;
                    }
                }
            }
            count + 2
        })
        .sum()
}

pub fn task2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().filter(|&c| c == '\\' || c == '"').count() + 2)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_task1() {
        assert_eq!(task1(EXAMPLE), 12);
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2(EXAMPLE), 18);
    }
}
