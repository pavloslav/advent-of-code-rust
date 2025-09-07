use std::collections::HashMap;

pub struct Part {
    rate: [i64; 4],
}

impl<'a> prse::Parse<'a> for Part {
    fn from_str(s: &'a str) -> Result<Self, prse::ParseError> {
        let (x, m, a, s) = prse::try_parse!(s, "{{x={},m={},a={},s={}}}")?;
        Ok(Part { rate: [x, m, a, s] })
    }
}

impl Part {
    fn rating(&self) -> i64 {
        self.rate.iter().sum()
    }
}

pub struct RateIndex(usize);

impl<'a> prse::Parse<'a> for RateIndex {
    fn from_str(s: &'a str) -> Result<Self, prse::ParseError> {
        match s {
            "x" => Ok(RateIndex(0)),
            "m" => Ok(RateIndex(1)),
            "a" => Ok(RateIndex(2)),
            "s" => Ok(RateIndex(3)),
            other => Err(prse::ParseError::Other(format!("Unknown index: {other}"))),
        }
    }
}

#[derive(prse::Parse)]
pub enum Rule {
    #[prse = "{rate_idx}<{value}:{target}"]
    Less {
        rate_idx: RateIndex,
        value: i64,
        target: String,
    },
    #[prse = "{rate_idx}>{value}:{target}"]
    Greater {
        rate_idx: RateIndex,
        value: i64,
        target: String,
    },
    #[prse = "{target}"]
    Always { target: String },
}

type Parts = Vec<Part>;
type Workflows = HashMap<String, Vec<Rule>>;

pub fn parse_input(input: &str) -> anyhow::Result<(Parts, Workflows)> {
    let mut parts = vec![];
    let mut rules = HashMap::new();
    for line in input.lines() {
        if !line.is_empty() {
            if let Ok(part) = prse::try_parse!(line, "{}") {
                parts.push(part);
            } else {
                let (name, workflows): (String, Vec<Rule>) = prse::try_parse!(line, "{}{{{:,:}}}")?;
                rules.insert(name, workflows);
            }
        }
    }

    Ok((parts, rules))
}

pub fn task1((parts, workflows): &(Parts, Workflows)) -> anyhow::Result<i64> {
    Ok(parts
        .iter()
        .map(|part| {
            let mut name = String::from("in");
            while let Some(rules) = workflows.get(&name) {
                for rule in rules {
                    match rule {
                        Rule::Greater {
                            rate_idx,
                            value,
                            target,
                        } if part.rate[rate_idx.0] > *value => {
                            name = target.clone();
                            break;
                        }
                        Rule::Less {
                            rate_idx,
                            value,
                            target,
                        } if part.rate[rate_idx.0] < *value => {
                            name = target.clone();
                            break;
                        }
                        Rule::Always { target } => {
                            name = target.clone();
                            break;
                        }
                        _ => {}
                    }
                }
            }
            if name == "A" { part.rating() } else { 0 }
        })
        .sum())
}

#[derive(Clone)]
struct PartRange {
    name: String,
    rule: usize,
    ratings: [std::ops::RangeInclusive<i64>; 4],
}

impl PartRange {
    fn new_full() -> Self {
        Self {
            name: String::from("in"),
            rule: 0,
            ratings: [1..=4000, 1..=4000, 1..=4000, 1..=4000],
        }
    }
    fn rating(&self) -> i64 {
        self.ratings
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .product()
    }
}

pub fn task2((_, workflows): &(Parts, Workflows)) -> anyhow::Result<i64> {
    let mut stack = vec![PartRange::new_full()];
    let mut result = 0;
    while let Some(part_range) = stack.pop() {
        if part_range.name == "A" {
            result += part_range.rating();
        } else if let Some(workflow) = workflows.get(&part_range.name)
            && let Some(rule) = workflow.get(part_range.rule)
        {
            match rule {
                Rule::Less {
                    rate_idx,
                    value,
                    target,
                } => {
                    let range = part_range.ratings[rate_idx.0].clone();
                    if range.start() < value {
                        let mut lower_range = part_range.clone();
                        lower_range.name = target.clone();
                        lower_range.rule = 0;
                        lower_range.ratings[rate_idx.0] =
                            *range.start()..=*range.end().min(value) - 1;
                        stack.push(lower_range);
                    }
                    if range.end() > value {
                        let mut upper_range = part_range.clone();
                        upper_range.rule += 1;
                        upper_range.ratings[rate_idx.0] = *range.start().max(value)..=*range.end();
                        stack.push(upper_range);
                    }
                }
                Rule::Greater {
                    rate_idx,
                    value,
                    target,
                } => {
                    let range = part_range.ratings[rate_idx.0].clone();
                    if range.start() <= value {
                        let mut lower_range = part_range.clone();
                        lower_range.rule += 1;
                        lower_range.ratings[rate_idx.0] = *range.start()..=*range.end().min(value);
                        stack.push(lower_range);
                    }
                    if range.end() > value {
                        let mut upper_range = part_range.clone();
                        upper_range.name = target.clone();
                        upper_range.rule = 0;
                        upper_range.ratings[rate_idx.0] =
                            *range.start().max(value) + 1..=*range.end();
                        stack.push(upper_range);
                    }
                }
                Rule::Always { target } => {
                    if target != "R" {
                        let mut range = part_range.clone();
                        range.name = target.clone();
                        range.rule = 0;
                        stack.push(range);
                    }
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1, task2};

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 19114);
    }
    #[test]
    fn test_task2() {
        assert_eq!(
            task2(&parse_input(INPUT).unwrap()).unwrap(),
            167409079868000
        );
    }
}
