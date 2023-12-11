pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{: :}")?))
        .collect()
}

pub fn task1(input: &[Vec<i32>]) -> anyhow::Result<i32> {
    Ok(input
        .iter()
        .map(|line| {
            let mut s = 0;
            let mut line = line.clone();
            while !line.iter().all(|&x| x == 0) {
                s += line.last().expect("unreachable");
                line = line
                    .iter()
                    .zip(line[1..].iter())
                    .map(|(a, b)| b - a)
                    .collect();
            }
            s
        })
        .sum())
}

pub fn task2(input: &[Vec<i32>]) -> anyhow::Result<i32> {
    Ok(input
        .iter()
        .map(|line| {
            let mut s = 0;
            let mut positive = true;
            let mut line = line.clone();
            while !line.iter().all(|&x| x == 0) {
                if positive {
                    s += line[0];
                } else {
                    s -= line[0];
                }
                positive = !positive;
                line = line
                    .iter()
                    .zip(line[1..].iter())
                    .map(|(a, b)| b - a)
                    .collect();
            }
            s
        })
        .sum())
}
