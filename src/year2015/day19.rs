use crate::*;
use std::collections::HashSet;

type Molecule = String;
type Rules = Vec<(Molecule, Molecule)>;

pub fn parse_input(input: &str) -> Result<(Rules, Molecule)> {
    let mut lines = input.lines();
    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| Ok(scan_fmt::scan_fmt!(line, "{} => {}", String, String)?))
        .collect::<Result<Rules>>()?;
    let molecule = lines
        .next()
        .ok_or(aoc_error!("No molecula for Rudolph"))?
        .to_string();
    Ok((rules, molecule))
}

pub fn task1((rules, medicine): &(Rules, Molecule)) -> Result<usize> {
    let mut molecules = HashSet::new();
    println!("len={}", rules.len());
    for (from, to) in rules {
        for (loc, _) in medicine.match_indices(from) {
            molecules.insert(format!(
                "{}{}{}",
                &medicine[..loc],
                &to,
                &medicine[loc + from.len()..]
            ));
        }
    }
    Ok(molecules.len())
}

pub fn task2((_rules, medicine): &(Rules, Molecule)) -> Result<usize> {
    /* Rules have three special elements: Rn, Ar and Y
     *  they are never on the left side and always in a pattern
     *  _ => _ Rn (_ Y)* _ Ar
     * The only other pattern is
     * _ => _ _ (two elements from one)
     * So, removing Y costs 1/2 operation, removing Ar or Rn - 1 operation
     * https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/
     */
    let y = medicine.matches('Y').count();
    let rn = medicine.matches("Rn").count();
    let ar = medicine.matches("Ar").count();
    if rn != ar {
        return Err(aoc_error!("Rn and Ar are not in symmetry!"));
    }
    let elements = medicine.matches(char::is_uppercase).count();
    Ok(elements - 2 * y - rn - ar - 1)
}
