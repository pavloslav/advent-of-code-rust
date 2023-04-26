use super::super::common::Result;

pub struct Cargo {
    stacks: Vec<Vec<char>>,
    plan: Vec<(usize, usize, usize)>,
}

pub fn parse_input(input: &str) -> Result<Cargo> {
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
        } else if let Ok((number, from, to)) = scan_fmt::scan_fmt!(
            line,
            "move {} from {} to {}",
            usize,
            usize,
            usize
        ) {
            plan.push((number, from, to));
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    Ok(Cargo { stacks, plan })
}

pub fn task1(input: &Cargo) -> Result<String> {
    let mut stacks = input.stacks.clone();
    for &(number, from, to) in &input.plan {
        for _ in 0..number {
            if let Some(crat) = stacks[from - 1].pop() {
                stacks[to - 1].push(crat);
            }
        }
    }
    Ok(stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' ').to_string())
        .collect())
}

pub fn task2(input: &Cargo) -> Result<String> {
    let mut stacks = input.stacks.clone();
    for &(number, from, to) in &input.plan {
        let new_from_len = stacks[from - 1].len() - number;
        let crates = Vec::from(&stacks[from - 1][new_from_len..]);
        stacks[from - 1].truncate(new_from_len);
        stacks[to - 1].extend(crates);
    }
    Ok(stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' ').to_string())
        .collect())
}
