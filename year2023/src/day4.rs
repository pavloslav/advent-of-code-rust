use std::collections::HashSet;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let (_num, win, matches): (&str, _, _) =
                prse::try_parse!(line, "Card {}: {: :0} | {: :0}")?;

            let win: HashSet<usize> = win
                .filter(|i| i != &Ok(""))
                .map(|i| Ok(i?.parse()?))
                .collect::<anyhow::Result<_>>()?;
            let matches = matches
                .filter(|i| i != &Ok(""))
                .map(|i| Ok(i?.parse()?))
                .collect::<anyhow::Result<_>>()?;

            Ok(win.intersection(&matches).count())
        })
        .collect()
}

fn win(&matches: &usize) -> usize {
    if matches == 0 {
        0
    } else {
        1 << (matches - 1)
    }
}

pub fn task1(input: &[usize]) -> anyhow::Result<usize> {
    Ok(input.iter().map(win).sum())
}

pub fn task2(input: &[usize]) -> anyhow::Result<usize> {
    let mut card_count = vec![1; input.len()];
    for (i, matches) in input.iter().enumerate() {
        let count = card_count[i];
        for j in &mut card_count[i + 1..=i + matches] {
            *j += count;
        }
    }

    Ok(card_count.iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test_task1() {
        let card = parse_input(INPUT).unwrap();
        assert_eq!(task1(&card).unwrap(), 13);
    }

    #[test]
    fn test_task2() {
        let card = parse_input(INPUT).unwrap();
        assert_eq!(task2(&card).unwrap(), 30);
    }
}
