use itertools::Itertools;
use prse::Parse;

#[derive(prse::Parse)]
#[prse = "{springs} {groups:,:}"]
pub struct Line<'a> {
    springs: &'a str,
    groups: Vec<usize>,
}

pub fn parse_input<'a>(input: &'a str) -> anyhow::Result<Vec<Line<'a>>> {
    Ok(input
        .lines()
        .map(Line::from_str)
        .collect::<Result<_, _>>()?)
}

const EMPTY: u8 = b'.';
const SPRING: u8 = b'#';
//const UNKNOWN: u8 = b'?';

fn arrangements(map: &[u8], groups: &[usize], sum: usize) -> usize {
    if let Some(first) = groups.first() {
        if let Some(mut max_position) = (map.len() + 1).checked_sub(groups.len() + sum) {
            if let Some(pos) = map.iter().position(|&c| c == SPRING) {
                max_position = max_position.min(pos);
            }
            (0..=max_position)
                .map(|i| {
                    if map[i..i + first].iter().all(|&c| c != EMPTY)
                        && map.get(i + first) != Some(&SPRING)
                    {
                        if map.len() == i + first
                            || (groups.len() == 1
                                && map[i + first + 1..].iter().all(|&c| c != SPRING))
                        {
                            if i == 0 || map.get(i - 1) != Some(&SPRING) {
                                1
                            } else {
                                0
                            }
                        } else if i == 0 || map.get(i - 1) != Some(&SPRING) {
                            arrangements(&map[i + first + 1..], &groups[1..], sum - first)
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum()
        } else {
            0
        }
    } else if map.iter().all(|&c| c != SPRING) {
        1
    } else {
        0
    }
}

pub fn task1(input: &[Line]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .map(|line| {
            arrangements(
                line.springs.as_bytes(),
                line.groups.as_ref(),
                line.groups.iter().sum(),
            )
        })
        .sum())
}

pub fn task2(input: &[Line]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .map(|line| {
            println!("{}", line.springs);
            let springs = std::iter::repeat_n(line.springs, 5).join("?");
            let groups: Vec<_> = std::iter::repeat_n(&line.groups, 5)
                .flatten()
                .copied()
                .collect();
            arrangements(
                springs.as_bytes(),
                groups.as_ref(),
                line.groups.iter().sum::<usize>() * 5,
            )
        })
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_task1() {
        for (line, expect) in INPUT.lines().zip([1, 4, 1, 1, 4, 10]) {
            assert_eq!(task1(&parse_input(line).unwrap()).unwrap(), expect);
        }

        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 21);
    }
}
