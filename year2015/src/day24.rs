use anyhow::Context;

type Weight = i64;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Weight>> {
    input
        .lines()
        .map(|s| Ok(s.parse()?))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn weight(presents: &[&Weight]) -> Weight {
    presents.iter().copied().sum()
}

fn qe(presents: &[&Weight]) -> Weight {
    presents.iter().copied().product()
}

use itertools::Itertools;

fn can_be_split_in(presents: &[Weight], n: Weight) -> bool {
    let mass: Weight = presents.iter().sum();
    if n < 2 {
        return true;
    }
    if mass % n == 0 {
        let target = mass / n;
        for comb in presents.iter().powerset() {
            if weight(&comb) == target {
                let other_presents: Vec<_> = presents
                    .iter()
                    .filter(|p| !comb.contains(p))
                    .copied()
                    .collect();
                return can_be_split_in(&other_presents, n - 1);
            }
        }
    }
    false
}

/*PREMATURE OPTIMIZATION IS THE ROOT OF ALL EVIL*/
fn task(presents: &[Weight], parts: Weight) -> anyhow::Result<Weight> {
    let mut best = None;
    let mass: Weight = presents.iter().sum();
    if mass % parts != 0 {
        anyhow::bail!("Can't divide {mass} by {parts}!");
    }
    let target = mass / parts;
    for len in 0..presents.len() {
        if best.is_some() {
            break;
        }

        for comb in presents.iter().combinations(len) {
            if weight(&comb) == target {
                let other_presents: Vec<_> = presents
                    .iter()
                    .filter(|p| !comb.contains(p))
                    .copied()
                    .collect();
                if can_be_split_in(&other_presents, parts - 1) {
                    let qe = Some(qe(&comb));
                    best = if best.is_some() { best.min(qe) } else { qe }
                }
            }
        }
    }
    best.context("Not found")
}

pub fn task1(input: &[Weight]) -> anyhow::Result<Weight> {
    task(input, 3)
}

pub fn task2(input: &[Weight]) -> anyhow::Result<Weight> {
    task(input, 4)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let data = [11, 10, 9, 8, 7, 5, 4, 3, 2, 1];
        assert_eq!(task1(&data).unwrap(), 99);
    }
}
