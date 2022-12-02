pub fn parse_input(input: &str) -> Vec<u32> {
    let mut result = vec![0];
    for calories in input.lines() {
        if let Ok(calories) = calories.parse::<u32>() {
            let idx = result.len() - 1;
            result[idx] += calories;
        } else {
            result.push(0);
        }
    }
    result.sort();
    result
}

pub fn task1(elves: &[u32]) -> u32 {
    elves[elves.len() - 1]
}

pub fn task2(elves: &[u32]) -> u32 {
    elves[elves.len() - 3..].iter().sum()
}
