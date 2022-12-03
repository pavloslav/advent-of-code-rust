pub fn parse_input(input: &str) -> &str {
    input
}

fn value(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => panic!("Wrong input"),
    }
}

pub fn task1(instructions: &str) -> i32 {
    instructions.chars().map(value).sum()
}

pub fn task2(instructions: &str) -> usize {
    instructions
        .chars()
        .scan(0, |acc, c| {
            *acc += value(c);
            Some(*acc).filter(|&floor| floor >= 0)
        })
        .count()
        + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1("))((((("), 3);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2("()())"), 5)
    }
}
