use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<i32>> {
    input.chars().map(value).collect()
}

fn value(c: char) -> Result<i32> {
    match c {
        '(' => Ok(1),
        ')' => Ok(-1),
        _ => Err(task_error!("Wrong symbol {c}")),
    }
}

pub fn task1(instructions: &[i32]) -> Result<i32> {
    Ok(instructions.iter().sum())
}

pub fn task2(instructions: &[i32]) -> Result<usize> {
    let not_in_basement = instructions
        .iter()
        .scan(0, |floor, change| {
            *floor += change;
            if *floor >= 0 {
                Some(*floor)
            } else {
                None
            }
        })
        .count();
    Ok(not_in_basement + 1)
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
