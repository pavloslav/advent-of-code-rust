use std::boxed::Box;
use std::str::Chars;

#[derive(Clone)]
pub enum SnailfishNumber {
    Regular(u32),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}

impl std::fmt::Debug for SnailfishNumber {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::Regular(value) => value.fmt(fmt),
            SnailfishNumber::Pair(pair) => {
                write!(fmt, "[{:?},{:?}]", &pair.0, &pair.1)
            }
        }
    }
}

impl SnailfishNumber {
    fn pair(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
        SnailfishNumber::Pair(Box::new((left, right)))
    }
    fn regular(value: u32) -> SnailfishNumber {
        SnailfishNumber::Regular(value)
    }
    fn regpair(left: u32, right: u32) -> SnailfishNumber {
        SnailfishNumber::pair(
            SnailfishNumber::regular(left),
            SnailfishNumber::regular(right),
        )
    }
}

fn read_snailfish_number<'a>(input: &mut Chars<'a>) -> SnailfishNumber {
    let first = input.next().unwrap();
    if first == '[' {
        let left = read_snailfish_number(input.by_ref());
        assert_eq!(input.next().unwrap(), ',');
        let right = read_snailfish_number(input.by_ref());
        assert_eq!(input.next().unwrap(), ']');
        SnailfishNumber::pair(left, right)
    } else {
        SnailfishNumber::regular(first.to_digit(10).unwrap())
    }
}

enum Explosion {
    Not,
    Both(u32, u32),
    FromLeft(u32),
    FromRight(u32),
    Did,
}

impl Explosion {
    fn to_bool(&self) -> bool {
        match self {
            Explosion::Not => false,
            _ => true,
        }
    }
}

impl SnailfishNumber {
    fn reduce(&mut self) {
        loop {
            //println!("Calling explode for {:?}", self);
            if !self.explode(1).to_bool() {
                //println!("Calling split for {:?}", self);
                if !self.split() {
                    //println!("Done: {:?}", self);
                    break;
                }
            }
        }
    }

    fn explode(&mut self, level: u32) -> Explosion {
        //println!("Exploding level={}: {:?}", level, self);
        match self {
            SnailfishNumber::Regular(_) => Explosion::Not,
            SnailfishNumber::Pair(pair) => {
                if let (
                    SnailfishNumber::Regular(left),
                    SnailfishNumber::Regular(right),
                ) = **pair
                {
                    if level > 4 {
                        Explosion::Both(left, right)
                    } else {
                        Explosion::Not
                    }
                } else {
                    match pair.0.explode(level + 1) {
                        Explosion::Both(left_left, left_right) => {
                            pair.0 = SnailfishNumber::regular(0);
                            pair.1.insert_from_left(left_right);
                            Explosion::FromLeft(left_left)
                        }
                        Explosion::FromRight(left_right) => {
                            pair.1.insert_from_left(left_right);
                            Explosion::Did
                        }
                        Explosion::Not => match pair.1.explode(level + 1) {
                            Explosion::Both(right_left, right_right) => {
                                pair.0.insert_from_right(right_left);
                                pair.1 = SnailfishNumber::regular(0);
                                Explosion::FromRight(right_right)
                            }
                            Explosion::FromLeft(right_left) => {
                                pair.0.insert_from_right(right_left);
                                Explosion::Did
                            }
                            other => other, //Did, Not, Right
                        },
                        other => other, //Did, Not, LEft
                    }
                }
            }
        }
    }

    fn insert_from_left(&mut self, value: u32) {
        //println!("Insert left {:?} into {:?}", value, self);
        match self {
            SnailfishNumber::Regular(self_value) => {
                *self_value += value;
            }
            SnailfishNumber::Pair(pair) => {
                pair.0.insert_from_left(value);
            }
        }
    }

    fn insert_from_right(&mut self, value: u32) {
        //println!("Insert right {:?} into {:?}", value, self);
        match self {
            SnailfishNumber::Regular(self_value) => {
                *self_value += value;
            }
            SnailfishNumber::Pair(pair) => {
                pair.1.insert_from_right(value);
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailfishNumber::Regular(value) => {
                if *value >= 10 {
                    *self =
                        SnailfishNumber::regpair(*value / 2, (*value + 1) / 2);
                    true
                } else {
                    false
                }
            }
            SnailfishNumber::Pair(pair) => pair.0.split() || pair.1.split(),
        }
    }

    fn magnitude(&self) -> u32 {
        //println!("Finding magnitude of {:?}", self);
        match self {
            SnailfishNumber::Regular(v) => *v,
            SnailfishNumber::Pair(pair) => {
                3 * pair.0.magnitude() + 2 * pair.1.magnitude()
            }
        }
    }
}

fn add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    let mut result = SnailfishNumber::pair(left, right);
    result.reduce();
    result
}

fn sum_vec(numbers: &Vec<SnailfishNumber>) -> SnailfishNumber {
    numbers
        .iter()
        .skip(1)
        .fold(numbers[0].clone(), |acc, num| add(acc, num.clone()))
}

pub fn parse_input(input: &str) -> Vec<SnailfishNumber> {
    input
        .lines()
        .map(|line| read_snailfish_number(&mut line.chars()))
        .collect()
}

pub fn task1(input: &Vec<SnailfishNumber>) -> u32 {
    sum_vec(input).magnitude()
}

pub fn task2(input: &Vec<SnailfishNumber>) -> u32 {
    let mut best = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                best = std::cmp::max(
                    best,
                    add(input[i].clone(), input[j].clone()).magnitude(),
                );
            }
        }
    }
    best
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reduce() {
        let data = "[[[[[9,8],1],2],3],4]
[[6,[5,[4,[3,2]]]],1]
[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let expected = "[[[[0,9],2],3],4]
[[6,[5,[7,0]]],3]
[[3,[2,[8,0]]],[9,[5,[7,0]]]]
[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let mut nums = parse_input(&data);
        for (input, expected) in nums.iter_mut().zip(expected.lines()) {
            input.reduce();
            assert_eq!(format!("{:?}", input), expected);
        }
    }

    #[test]
    fn test_magnitude() {
        for (input, expected) in parse_input(
            "[[1,2],[[3,4],5]]
[[[[0,7],4],[[7,8],[6,0]]],[8,1]]
[[[[1,1],[2,2]],[3,3]],[4,4]]
[[[[3,0],[5,3]],[4,4]],[5,5]]
[[[[5,0],[7,4]],[5,5]],[6,6]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        )
        .iter()
        .zip([143, 1384, 445, 791, 1137, 3488].into_iter())
        {
            assert_eq!(input.magnitude(), expected);
        }
    }

    #[test]
    fn test_add1() {
        let nums = &parse_input(
            "[1,1]
[2,2]",
        );
        let sum = add(nums[0].clone(), nums[1].clone());
        assert_eq!(format!("{:?}", sum), "[[1,1],[2,2]]");
    }

    #[test]
    fn test_add3() {
        let nums = &parse_input(
            "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]",
        );
        let sum = add(nums[0].clone(), nums[1].clone());
        assert_eq!(format!("{:?}", sum), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_add2() {
        let nums = &parse_input(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        );
        let sum = add(nums[0].clone(), nums[1].clone());
        assert_eq!(
            format!("{:?}", sum),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
    }

    #[test]
    fn test_sum1() {
        let data = "[1,1]
[2,2]
[3,3]
[4,4]";
        let num = parse_input(&data);
        let result = sum_vec(&num);
        assert_eq!(format!("{:?}", &result), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_sum2() {
        let data = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";
        let num = parse_input(&data);
        let result = sum_vec(&num);
        assert_eq!(format!("{:?}", &result), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_sum3() {
        let data = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
        let num = parse_input(&data);
        let result = sum_vec(&num);
        assert_eq!(format!("{:?}", &result), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_task1() {
        let data = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

        assert_eq!(task1(&parse_input(&data)), 4140);
    }
}
