use std::collections::HashSet;

const EMPTY: u8 = b'.';
const PLANT: u8 = b'#';

fn str_to_plant(input: &str) -> anyhow::Result<Vec<bool>> {
    input
        .bytes()
        .map(|c| match c {
            PLANT => Ok(true),
            EMPTY => Ok(false),
            _ => Err(anyhow::anyhow!("Invalid plant state '{}'", c as char)),
        })
        .collect()
}

pub fn parse_input(input: &str) -> anyhow::Result<(Vec<bool>, HashSet<Vec<bool>>)> {
    let mut input = input.lines();
    let initial = input.next().ok_or(anyhow::anyhow!("No initial state!"))?;
    let initial = str_to_plant(prse::try_parse!(initial, "initial state: {}")?)?;

    let rules: HashSet<Vec<bool>> = input
        .skip(1)
        .filter_map(|line| {
            let (from, to): (&str, char) = match prse::try_parse!(line, "{} => {}") {
                Ok(v) => v,
                Err(e) => return Some(Err(e.into())),
            };

            match to.try_into() {
                Ok(PLANT) => Some(str_to_plant(from)),
                Ok(EMPTY) => None,
                _ => Some(Err(anyhow::anyhow!("Invalid plant state '{to}'"))),
            }
        })
        .collect::<anyhow::Result<_>>()?;
    if rules.iter().any(|rule| rule.len() != 5) {
        Err(anyhow::anyhow!("All rules must be of length 5!"))
    } else if rules.iter().any(|rule| rule.as_slice() == [false; 5]) {
        Err(anyhow::anyhow!("Empty rule leads to chaos!"))
    } else {
        Ok((initial, rules))
    }
}

pub fn task1((initial, rules): &(Vec<bool>, HashSet<Vec<bool>>)) -> anyhow::Result<isize> {
    let loops = 20;
    let mut state: Vec<bool> = vec![false; 5];
    state.extend_from_slice(initial);
    state.extend_from_slice(&[false; 5]);
    for _ in 0..20 {
        let mut new_state = vec![false; 5];
        new_state.extend((0..state.len() - 5).map(|i| rules.contains(&state[i..i + 5])));
        new_state.extend_from_slice(&[false; 5]);
        state = new_state;
    }
    Ok(state
        .iter()
        .enumerate()
        .filter_map(|(i, &pot)| {
            if pot {
                Some(i as isize - 3 * loops - 5)
            } else {
                None
            }
        })
        .sum())
}

pub fn task2((_initial, _rules): &(Vec<bool>, HashSet<Vec<bool>>)) -> anyhow::Result<usize> {
    Err(anyhow::anyhow!("Todo"))
}
