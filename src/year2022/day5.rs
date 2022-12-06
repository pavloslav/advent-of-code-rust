pub struct Cargo {
    stacks: Vec<Vec<char>>,
    plan: Vec<(usize, usize, usize)>,
}

pub fn parse_input(input: &str) -> Cargo {
    let mut stacks = Vec::new();
    let mut plan = Vec::new();
    for line in input.lines() {
        if line.contains('[') {
            if stacks.len() < line.len() / 4 {
                stacks.resize(line.len() / 4 + 1, Vec::new());
            }
            for (i, stack) in stacks.iter_mut().enumerate() {
                if let Some(crat) = line.chars().nth(i * 4 + 1) {
                    if crat != ' ' {
                        stack.push(crat);
                    }
                }
            }
        } else if line.starts_with("move") {
            let mut line = line.split_whitespace();
            line.next();
            let number = line.next().unwrap().parse().unwrap();
            line.next();
            let from = line.next().unwrap().parse().unwrap();
            line.next();
            let to = line.next().unwrap().parse().unwrap();
            plan.push((number, from, to));
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    Cargo { stacks, plan }
}

pub fn task1(input: &Cargo) -> String {
    let mut stacks = input.stacks.clone();
    for &(number, from, to) in &input.plan {
        for _ in 0..number {
            if let Some(crat) = stacks[from - 1].pop() {
                stacks[to - 1].push(crat);
            }
        }
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' ').to_string())
        .collect()
}

pub fn task2(input: &Cargo) -> String {
    let mut stacks = input.stacks.clone();
    for &(number, from, to) in &input.plan {
        let new_from_len = stacks[from - 1].len() - number;
        let crates = Vec::from(&stacks[from - 1][new_from_len..]);
        stacks[from - 1].truncate(new_from_len);
        stacks[to - 1].extend(crates);
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' ').to_string())
        .collect()
}
