use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<&str>> {
    Ok(input.lines().collect())
}

pub fn task1(input: &[&str]) -> AocResult<usize> {
    let mut sum = 0;

    for (y, &line) in input.iter().enumerate() {
        let mut x = line
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(line.len());
        while x < line.len() {
            let x_last = x + line[x..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(line[x..].len());
            let left = x.saturating_sub(1);
            let right = (x_last + 1).min(line.len());
            let top = y.saturating_sub(1);
            let bottom = (y + 2).min(input.len());
            let has_symbol = input[top..bottom]
                .iter()
                .flat_map(|l| l[left..right].chars().to_owned())
                .any(|c| c != '.' && !c.is_ascii_digit());
            if has_symbol {
                sum += line[x..x_last].parse::<usize>().map_err(|e| {
                    aoc_error!("Can't parse the number '{}': {}", &line[x..x_last], e)
                })?;
            }
            x = x_last
                + line[x_last..]
                    .find(|c: char| c.is_ascii_digit())
                    .unwrap_or(line[x_last..].len());
        }
    }
    Ok(sum)
}

pub fn task2(input: &[&str]) -> AocResult<usize> {
    let mut gears = std::collections::HashMap::new();

    for (y, &line) in input.iter().enumerate() {
        let mut x = line
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(line.len());
        while x < line.len() {
            let x_last = x + line[x..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(line[x..].len());
            let left = x.saturating_sub(1);
            let right = (x_last + 1).min(line.len());
            let top = y.saturating_sub(1);
            let bottom = (y + 2).min(input.len());
            let value = line[x..x_last]
                .parse::<usize>()
                .map_err(|e| aoc_error!("Can't parse the number '{}': {}", &line[x..x_last], e))?;
            for (dy, line) in input[top..bottom].iter().enumerate() {
                for (dx, c) in line[left..right].chars().enumerate() {
                    if c == '*' {
                        gears
                            .entry((left + dx, top + dy))
                            .and_modify(|v: &mut Vec<_>| v.push(value))
                            .or_insert_with(|| vec![value]);
                    }
                }
            }
            x = x_last
                + line[x_last..]
                    .find(|c: char| c.is_ascii_digit())
                    .unwrap_or(line[x_last..].len());
        }
    }
    Ok(gears
        .values()
        .filter_map(|g| {
            if g.len() == 2 {
                Some(g[0] * g[1])
            } else {
                None
            }
        })
        .sum())
}
