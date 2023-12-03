use crate::*;

type Tower = std::collections::HashMap<String, (usize, Vec<String>)>;

pub fn parse_input(input: &str) -> AocResult<Tower> {
    input
        .lines()
        .map(|line| {
            if let Ok((name, weight, children)) =
                prse::try_parse!(line, "{} ({d}) -> {[a-z, ]}", String, usize, String)
            {
                Ok((
                    name,
                    (weight, children.split(", ").map(|s| s.to_owned()).collect()),
                ))
            } else if let Ok((name, weight)) = prse::try_parse!(line, "{} ({d})", String, usize) {
                Ok((name, (weight, vec![])))
            } else {
                Err(aoc_error!("Failed to parse the line '{line}'"))
            }
        })
        .collect()
}

fn get_root(tower: &Tower) -> AocResult<String> {
    let children: std::collections::HashSet<_> = tower
        .values()
        .flat_map(|(_, children)| children.iter())
        .collect();
    tower
        .keys()
        .find(|k| !children.contains(k))
        .ok_or_else(|| aoc_error!("No root found!"))
        .cloned()
}

pub fn task1(tower: &Tower) -> AocResult<String> {
    get_root(tower)
}

fn get_weight(tower: &Tower, node: &str) -> usize {
    tower[node].0
        + tower[node]
            .1
            .iter()
            .map(|c| get_weight(tower, c))
            .sum::<usize>()
}

fn get_correct_weight(tower: &Tower, root: &str) -> AocResult<usize> {
    let mut weights = std::collections::HashMap::<usize, Vec<String>>::new();
    for child in &tower[root].1 {
        let weight = get_weight(tower, child);
        weights
            .entry(weight)
            .and_modify(|e| e.push(child.clone()))
            .or_insert_with(|| vec![child.clone()]);
    }
    if weights.len() == 2 {
        let mut correct = 0;
        let mut wrong = 0;
        for (&w, children) in &weights {
            if children.len() == 1 {
                wrong = w;
            } else {
                correct = w;
            }
        }

        get_correct_weight(tower, &weights[&wrong][0])
            .or_else(|_| Ok(tower[&weights[&wrong][0]].0 + correct - wrong))
    } else {
        Err(aoc_error!("Can't find answer"))
    }
}

pub fn task2(tower: &Tower) -> AocResult<usize> {
    get_correct_weight(tower, &get_root(tower)?)
}
