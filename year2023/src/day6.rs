use anyhow::Context;
use itertools::Itertools;

pub fn parse_input(input: &str) -> anyhow::Result<(&str, &str)> {
    let mut input = input.lines();
    let times = prse::try_parse!(input.next().context("Empty times")?, "Time: {}")?;
    let distances = prse::try_parse!(input.next().context("Empty distances")?, "Distance: {}")?;
    Ok((times, distances))
}

fn inequation(time: f64, dist: f64) -> i32 {
    // hold * (time - hold) > dist
    // hold.pow(2) - hold * time + dist < 0
    // D = time.pow(2) - 4 * dist
    // hold = (time ± D.sqrt()) / 2
    let discriminant = time * time - 4.0 * dist;
    if discriminant.is_sign_negative() {
        0
    } else {
        let sq = discriminant.sqrt();
        let hold1 = (time - sq) / 2.0;
        let hold2 = (time + sq) / 2.0;
        // inequation is strict, it means we must skip the roots
        // thus (hold2.ceil() - 1) - (hold1.floor() + 1) + 1
        hold2.ceil() as i32 - hold1.floor() as i32 - 1
    }
}

pub fn task1((times, distances): &(&str, &str)) -> anyhow::Result<i32> {
    times
        .split_whitespace()
        .map(|t| t.parse())
        .zip(distances.split_whitespace().map(|d| d.parse()))
        .map(|(time, dist)| Ok(inequation(time?, dist?)))
        .try_fold(1, |acc, r: anyhow::Result<_>| Ok(acc * r?))
}

pub fn task2((time, distance): &(&str, &str)) -> anyhow::Result<i32> {
    let time = time.split_whitespace().join("").parse()?;
    let distance = distance.split_whitespace().join("").parse()?;
    Ok(inequation(time, distance))
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1, task2};
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 288);
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input(INPUT).unwrap()).unwrap(), 71503);
    }
}
