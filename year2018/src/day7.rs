use std::collections::{HashMap, HashSet /* , VecDeque*/};

pub fn parse_input(input: &str) -> anyhow::Result<Vec<(char, char)>> {
    input
        .lines()
        .map(|line| {
            Ok(prse::try_parse!(
                line,
                "Step {} must be finished before step {} can begin."
            )?)
        })
        .collect()
}

pub fn task1(input: &[(char, char)]) -> anyhow::Result<String> {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();
    for &(needed, target) in input {
        map.entry(target)
            .and_modify(|e| {
                e.insert(needed);
            })
            .or_insert([needed].into());
        map.entry(needed).or_default();
    }
    let mut steps = String::new();
    while !map.is_empty() {
        let next_step = map
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(&k, _)| k)
            .min()
            .ok_or(anyhow::anyhow!("No suitable action found!"))?;
        steps.push(next_step);
        for v in map.values_mut() {
            v.remove(&next_step);
        }
        map.remove(&next_step);
    }
    Ok(steps)
}

pub fn task2(_input: &[(char, char)]) -> anyhow::Result<usize> {
    anyhow::bail!("Todo")
    /*
        let mut steps: HashSet<char> = input
            .iter()
            .flat_map(|&(needed, target)| [needed, target])
            .collect();
        let mut map: HashMap<char, Vec<char>> = steps.iter().map(|&step| (step, vec![])).collect();
        for &(needed, target) in input {
            if let Some(targets) = map.get_mut(&needed) {
                targets.push(target);
            }
        }

        let mut queue: VecDeque<(char, usize)> = VecDeque::new();
        while !steps.is_empty() {
            let mut availiable = vec![];
            for &step in &steps {
                if map[&step].is_empty() {

                }
                if let Some(min) = map[&step]
                    .iter()
                    .map(|target| times.get(target))
                    .min()
                    .map_or(Some(0), |rest| {
                        rest.map(|t| t + 61 + step as usize - 'A' as usize)
                    })
                {
                    times.insert(step, min);
                    to_remove.push(step);
                }
            }
            for r in to_remove {
                steps.remove(&r);
            }
        }
    */
}
