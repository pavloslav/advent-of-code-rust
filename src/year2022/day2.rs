pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

// score for the second player
fn outcome(line: &str) -> i32 {
    match line {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => panic!(),
    }
}

pub fn task1(input: &[&str]) -> i32 {
    input.iter().map(|line| outcome(line)).sum()
}

fn correct_outcome(line: &str) -> i32 {
    match line {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => panic!(),
    }
}

pub fn task2(input: &[&str]) -> i32 {
    input.iter().map(|line| correct_outcome(line)).sum()
}
