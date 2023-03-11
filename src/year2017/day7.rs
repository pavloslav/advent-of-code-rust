use once_cell::sync::Lazy;

type Tower = std::collections::HashMap<String, (usize, Vec<String>)>;

pub fn parse_input(input: &str) -> Tower {
    static INPUT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(
            r"(?P<name>\w+) \((?P<weight>\d+)\)( -> (?P<child>.*))?",
        )
        .unwrap()
    });
    input
        .lines()
        .map(|line| {
            if let Some(captures) = INPUT_REGEX.captures(line) {
                if let (Some(name), Some(Ok(weight))) = (
                    captures.name("name"),
                    captures.name("weight").map(|w| w.as_str().parse()),
                ) {
                    let children =
                        if let Some(children) = captures.name("child") {
                            children
                                .as_str()
                                .split(", ")
                                .map(|s| s.to_owned())
                                .collect()
                        } else {
                            vec![]
                        };
                    (name.as_str().to_owned(), (weight, children))
                } else {
                    panic!("No name and weight found in '{line}'")
                }
            } else {
                panic!("Failed to parse the line '{line}'")
            }
        })
        .collect()
}

fn get_root(tower: &Tower) -> String {
    let children: std::collections::HashSet<_> = tower
        .values()
        .flat_map(|(_, children)| children.iter())
        .collect();
    tower
        .keys()
        .find(|k| !children.contains(k))
        .unwrap()
        .clone()
}

pub fn task1(tower: &Tower) -> String {
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

fn get_correct_weight(tower: &Tower, root: &str) -> Option<usize> {
    let mut weights = std::collections::HashMap::<usize, Vec<String>>::new();
    for child in &tower[root].1 {
        let weight = get_weight(tower, child);
        weights.entry(weight).or_insert(vec![]).push(child.clone());
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
            .or(Some(tower[&weights[&wrong][0]].0 + correct - wrong))
    } else {
        None
    }
}

pub fn task2(tower: &Tower) -> usize {
    get_correct_weight(tower, &get_root(tower)).unwrap()
}
