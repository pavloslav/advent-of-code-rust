use std::collections::HashSet;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    input.lines().map(|line| Ok(line.parse()?)).collect()
}

pub fn task1(frequencies: &[i32]) -> anyhow::Result<i32> {
    Ok(frequencies.iter().sum())
}

pub fn task2(frequencies: &[i32]) -> anyhow::Result<i32> {
    let mut f = 0;
    let mut visited = HashSet::from([0]);
    for change in frequencies.iter().cycle() {
        f += change;
        if !visited.insert(f) {
            return Ok(f);
        }
    }
    Err(anyhow::anyhow!("unreachable!"))
}
