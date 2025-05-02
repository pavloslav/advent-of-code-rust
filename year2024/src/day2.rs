use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| prse::try_parse!(line, "{: :}").context("Error parsing line"))
        .collect()
}

fn is_safe(levels: &[i32], has_dampner: bool) -> bool {
    let mut dir_counter = std::collections::HashMap::new();
    for w in levels.windows(2).take(5) {
        dir_counter
            .entry(w[0].cmp(&w[1]))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let sign = if let Some(sign) = dir_counter.iter().find(move |&(_, &v)| v >= 3) {
        *sign.0
    } else {
        return false;
    };

    let tolerate = |a: i32, b: i32| a.cmp(&b) == sign && (a - b).abs() <= 3;
    let mut skipped = levels.len();

    for i in 0..levels.len() - 1 {
        if i != skipped && !tolerate(levels[i], levels[i + 1]) {
            if skipped != levels.len() || !has_dampner {
                return false;
            }
            skipped = if i != 0 && tolerate(levels[i - 1], levels[i + 1]) {
                i
            } else if i < levels.len() - 2 && tolerate(levels[i], levels[i + 2]) {
                i + 1
            } else {
                return false;
            }
        }
    }
    true
}

pub fn task1(input: &[Vec<i32>]) -> anyhow::Result<usize> {
    Ok(input.iter().filter(|report| is_safe(report, false)).count())
}

//468 - hi
//463 - low
pub fn task2(input: &[Vec<i32>]) -> anyhow::Result<usize> {
    Ok(input.iter().filter(|report| is_safe(report, true)).count())
}
