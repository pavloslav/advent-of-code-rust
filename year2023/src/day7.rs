use crate::*;

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
    fn new(cards: &[u8]) -> Self {
        let mut cnt = std::collections::HashMap::new();
        for card in cards {
            cnt.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }
        if cnt.values().any(|&c| c == 5) {
            HandKind::Five
        } else if cnt.values().any(|&c| c == 4) {
            HandKind::Four
        } else {
            let three = cnt.values().any(|&c| c == 3);
            let two = cnt.values().filter(|&&c| c == 2).count();
            if three {
                if two == 1 {
                    HandKind::FullHouse
                } else {
                    HandKind::Three
                }
            } else if two == 2 {
                HandKind::TwoPair
            } else if two == 1 {
                HandKind::OnePair
            } else {
                HandKind::High
            }
        }
    }
    fn new_with_jokers(cards: &[u8]) -> Self {
        let mut cnt = std::collections::HashMap::new();
        for card in cards {
            cnt.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }
        let jokers = cnt.remove(&JOKER).unwrap_or_default();
        let max = cnt.values().copied().max().unwrap_or_default();
        if jokers + max == 5 {
            HandKind::Five
        } else if jokers + max == 4 {
            HandKind::Four
        } else {
            let two = cnt.values().filter(|&&c| c == 2).count();
            if max == 3 && two == 1 || jokers == 1 && two == 2 {
                HandKind::FullHouse
            } else if jokers + max == 3 {
                HandKind::Three
            } else if two == 2 || jokers == 1 && two == 1 {
                HandKind::TwoPair
            } else if two == 1 || jokers == 1 {
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

const JOKER: u8 = 1;
const TEN: u8 = 10;
const JACK: u8 = 11;
const QUEEN: u8 = 12;
const KING: u8 = 13;
const ACE: u8 = 14;

impl Hand {
    fn try_new(cards: &str, bid: usize, with_jokers: bool) -> AocResult<Self> {
        if cards.len() != 5 {
            return Err(aoc_error!("Invalid number of cards: {}", cards.len()));
        }
        let cards: Vec<u8> = cards
            .bytes()
            .map(|i| {
                Ok(match i {
                    b'2'..=b'9' => i - b'0',
                    b'T' => TEN,
                    b'J' => {
                        if with_jokers {
                            JOKER
                        } else {
                            JACK
                        }
                    }
                    b'Q' => QUEEN,
                    b'K' => KING,
                    b'A' => ACE,
                    other => return Err(aoc_error!("Invalid card: {}", other as char)),
                })
            })
            .collect::<AocResult<_>>()?;

        let kind = if with_jokers {
            HandKind::new_with_jokers(&cards)
        } else {
            HandKind::new(&cards)
        };

        Ok(Self { cards, bid, kind })
    }
}

pub fn parse_input(input: &str) -> AocResult<Vec<(&str, usize)>> {
    input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{} {}")?))
        .collect()
}

fn task(input: &[(&str, usize)], with_jokers: bool) -> AocResult<usize> {
    let mut hands: Vec<Hand> = input
        .iter()
        .map(|&(cards, bid)| Hand::try_new(cards, bid, with_jokers))
        .collect::<AocResult<_>>()?;
    hands.sort();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum())
}

pub fn task1(input: &[(&str, usize)]) -> AocResult<usize> {
    task(input, false)
}

pub fn task2(input: &[(&str, usize)]) -> AocResult<usize> {
    task(input, true)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn test_task1() {
        assert!(
            Hand::try_new("33332", 1, false).unwrap() > Hand::try_new("2AAAA", 2, false).unwrap()
        );
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 6440);
    }

    #[test]
    fn test_task2() {
        assert!(
            Hand::try_new("QQQQ2", 1, false).unwrap() > Hand::try_new("JKKK2", 2, true).unwrap()
        );
        assert_eq!(task2(&parse_input(INPUT).unwrap()).unwrap(), 5905);
    }
}
