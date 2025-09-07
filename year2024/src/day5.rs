use std::{cmp::Ordering, collections::HashSet};

pub fn parse_input(input: &str) -> anyhow::Result<(HashSet<(i32, i32)>, Vec<Vec<i32>>)> {
    let mut lines = input.lines();
    let rules: HashSet<_> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| prse::try_parse!(line, "{}|{}"))
        .collect::<Result<_, _>>()?;
    let updates: Vec<_> = lines
        .map(|line| prse::try_parse!(line, "{:,:}"))
        .collect::<Result<_, _>>()?;
    Ok((rules, updates))
}

pub fn task1((rules, updates): &(HashSet<(i32, i32)>, Vec<Vec<i32>>)) -> anyhow::Result<i32> {
    Ok(updates
        .iter()
        .filter_map(|update| {
            update
                .is_sorted_by(|&a, &b| !rules.contains(&(b, a)))
                .then(|| update[update.len() / 2])
        })
        .sum())
}

pub fn task2((rules, updates): &(HashSet<(i32, i32)>, Vec<Vec<i32>>)) -> anyhow::Result<i32> {
    Ok(updates
        .iter()
        .filter_map(|update| {
            (!update.is_sorted_by(|&a, &b| !rules.contains(&(b, a)))).then(|| {
                let mut update = update.clone();
                let middle = update.len() / 2;
                *update
                    .select_nth_unstable_by(middle, |&a, &b| {
                        if a == b {
                            Ordering::Equal
                        } else if rules.contains(&(a, b)) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                    .1
            })
        })
        .sum())
}
