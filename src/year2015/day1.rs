use super::super::common::Result;
use super::Error::TaskError;
type Data = Vec<i32>;

pub fn parse_input(input: &str) -> Result<Data> {
    input.chars().map(value).collect()
}

fn value(c: char) -> Result<i32> {
    match c {
        '(' => Ok(1),
        ')' => Ok(-1),
        _ => Err(TaskError(format!("Wrong symbol {c}"))),
    }
}

pub fn task1(instructions: &Data) -> Result<i32> {
    Ok(instructions.iter().sum())
}

pub fn task2(instructions: &Data) -> Result<usize> {
    Ok(instructions
        .iter()
        .scan(0, |acc, c| {
            *acc += c;
            Some(*acc).filter(|&floor| floor >= 0)
        })
        .count()
        + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input("))(((((").unwrap()).unwrap(), 3);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input("()())").unwrap()).unwrap(), 5);
    }
}
