pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(instructions: &str) -> i32 {
    instructions.chars()
                .map(|c|match c {
                    '(' =>  1,
                    ')' => -1,
                    _ => panic!("Wrong input"),
                })
                .sum()
}

pub fn task2(instructions: &str) -> usize {
    instructions.chars()
                .scan(0, |acc, c| { 
                    *acc += match c {
                        '(' =>  1,
                        ')' => -1,
                        _ => panic!("Wrong input"),
                    };
                    Some(*acc).filter(|&floor|floor>=0)
                })
                .count() + 1
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