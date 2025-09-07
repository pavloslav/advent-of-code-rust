const JOKER: u8 = 1;
const TEN: u8 = 10;
const JACK: u8 = 11;
const QUEEN: u8 = 12;
const KING: u8 = 13;
const ACE: u8 = 14;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
enum Jokers {
    Absent = JACK,
    Present = JOKER,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandKind {
    fn new(cards: &[u8], with_jokers: Jokers) -> Self {
        let mut cnt = std::collections::HashMap::new();
        for card in cards {
            cnt.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }
        if with_jokers == Jokers::Present && cnt.get(&JOKER) != Some(&5) {
            let jokers = cnt.remove(&JOKER).unwrap_or_default();
            let max = cnt.values().copied().max().unwrap_or_default();
            for v in cnt.values_mut() {
                if *v == max {
                    *v += jokers;
                    break;
                }
            }
        }
        if cnt.values().len() == 1 {
            HandKind::Five
        } else if cnt.values().any(|&c| c == 4) {
            HandKind::Four
        } else if cnt.len() == 2 {
            HandKind::FullHouse
        } else {
            let three = cnt.values().any(|&c| c == 3);
            let two = cnt.values().filter(|&&c| c == 2).count();
            if three {
                HandKind::Three
            } else if two == 2 {
                HandKind::TwoPair
            } else if two == 1 {
                HandKind::OnePair
            } else {
                HandKind::High
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Hand {
    kind: HandKind,
    cards: Vec<u8>,
    bid: usize,
}

impl Hand {
    fn try_new(cards: &str, bid: usize, with_jokers: Jokers) -> anyhow::Result<Self> {
        if cards.len() != 5 {
            anyhow::bail!("Invalid number of cards: {}", cards.len());
        }
        let cards: Vec<u8> = cards
            .bytes()
            .map(|i| {
                Ok(match i {
                    b'2'..=b'9' => i - b'0',
                    b'T' => TEN,
                    b'J' => with_jokers as u8,
                    b'Q' => QUEEN,
                    b'K' => KING,
                    b'A' => ACE,
                    other => anyhow::bail!("Invalid card: {}", other as char),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        let kind = HandKind::new(&cards, with_jokers);

        Ok(Self { cards, bid, kind })
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<(&str, usize)>> {
    input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{} {}")?))
        .collect()
}

fn task(input: &[(&str, usize)], with_jokers: Jokers) -> anyhow::Result<usize> {
    let mut hands: Vec<Hand> = input
        .iter()
        .map(|&(cards, bid)| Hand::try_new(cards, bid, with_jokers))
        .collect::<anyhow::Result<_>>()?;
    hands.sort();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum())
}

pub fn task1(input: &[(&str, usize)]) -> anyhow::Result<usize> {
    task(input, Jokers::Absent)
}

pub fn task2(input: &[(&str, usize)]) -> anyhow::Result<usize> {
    task(input, Jokers::Present)
}

#[cfg(test)]
mod test {
    use super::{Hand, Jokers, parse_input, task1, task2};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn test_task1() {
        assert!(
            Hand::try_new("33332", 1, Jokers::Absent).unwrap()
                > Hand::try_new("2AAAA", 2, Jokers::Absent).unwrap()
        );
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 6440);
    }

    #[test]
    fn test_task2() {
        assert!(
            Hand::try_new("QQQQ2", 1, Jokers::Present).unwrap()
                > Hand::try_new("JKKK2", 2, Jokers::Present).unwrap()
        );
        assert_eq!(task2(&parse_input(INPUT).unwrap()).unwrap(), 5905);
    }
}
