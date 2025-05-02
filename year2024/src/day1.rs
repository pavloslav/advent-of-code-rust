pub fn parse_input(input: &str) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for pair in input.lines().map(|line| prse::try_parse!(line, "{} {}")) {
        let (l, r) = pair?;
        left.push(l);
        right.push(r);
    }
    Ok((left, right))
}

pub fn task1((left, right): &(Vec<i32>, Vec<i32>)) -> anyhow::Result<i32> {
    let mut left = left.clone();
    left.sort();
    let mut right = right.clone();
    right.sort();
    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum())
}

pub fn task2((left, right): &(Vec<i32>, Vec<i32>)) -> anyhow::Result<i32> {
    let mut count = std::collections::HashMap::new();
    for i in right {
        count.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    Ok(left.iter().map(|l| count.get(&l).unwrap_or(&0) * l).sum())
}
