use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| prse::try_parse!(line, "{: :}").context("Error parsing line"))
        .collect()
}

fn first_fail(levels: &[i32], increase: bool, skip: Option<usize>) -> Option<usize> {
    let mut prev = None;
    for (i, &x) in levels.iter().enumerate() {
        if Some(i) == skip {
            continue;
        }
        if let Some(prev) = prev
            && (increase != (x > prev) || x == prev || x.abs_diff(prev) > 3)
        {
            return Some(i);
        }
        prev = Some(x);
    }
    None
}

pub fn task1(input: &[Vec<i32>]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .filter(|report| {
            [true, false]
                .iter()
                .any(|&increase| first_fail(report, increase, None).is_none())
        })
        .count())
}

//468 - hi
//463 - low
pub fn task2(input: &[Vec<i32>]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .filter(|report| {
            [true, false].iter().any(|&increase| {
                if let Some(fail) = first_fail(report, increase, None) {
                    (fail.max(1) - 1..fail + 1)
                        .any(|f| first_fail(report, increase, Some(f)).is_none())
                } else {
                    true
                }
            })
        })
        .count())
}
