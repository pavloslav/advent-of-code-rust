use super::super::common::Result;
use super::Error::TaskError;

#[derive(Clone, Debug)]
pub enum Wire {
    Name(String),
    Value(u16),
}

impl Wire {
    fn new(input: &str) -> Wire {
        match input.parse() {
            Ok(value) => Wire::Value(value),
            Err(_) => Wire::Name(input.to_string()),
        }
    }

    fn calculate(&self, wires: &mut Wires) -> Result<u16> {
        match self {
            Wire::Name(name) => {
                let value = get_rule(wires, name)?.calculate(wires)?;
                wires.insert(name.clone(), Rule::Direct(Wire::Value(value)));
                Ok(value)
            }
            Wire::Value(value) => Ok(*value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Rule {
    And(Wire, Wire),
    Or(Wire, Wire),
    LShift(Wire, Wire),
    RShift(Wire, Wire),
    Not(Wire),
    Direct(Wire),
}

impl Rule {
    fn new(input: &str) -> Result<(String, Rule)> {
        static INPUT_REGEX: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| {
                regex::Regex::new(r"^((?P<left>\w+) (?P<op>\w+) (?P<right>\w+)|(NOT (?P<not>\w+))|(?P<value>\w+)) -> (?P<target>\w+)$").unwrap()
            });
        INPUT_REGEX
            .captures(input)
            .map(|captures| {
                if let (Some(left), Some(op), Some(right), Some(target)) = (
                    captures.name("left"),
                    captures.name("op"),
                    captures.name("right"),
                    captures.name("target"),
                ) {
                    let rule = match op.as_str() {
                        "AND" => Rule::And(
                            Wire::new(left.as_str()),
                            Wire::new(right.as_str()),
                        ),
                        "OR" => Rule::Or(
                            Wire::new(left.as_str()),
                            Wire::new(right.as_str()),
                        ),
                        "LSHIFT" => Rule::LShift(
                            Wire::new(left.as_str()),
                            Wire::new(right.as_str()),
                        ),
                        "RSHIFT" => Rule::RShift(
                            Wire::new(left.as_str()),
                            Wire::new(right.as_str()),
                        ),
                        other => {
                            return Err(TaskError(format!(
                                "Incorrect binary operation: {other}"
                            )))
                        }
                    };
                    Ok((target.as_str().to_string(), rule))
                } else if let (Some(not), Some(target)) =
                    (captures.name("not"), captures.name("target"))
                {
                    Ok((
                        target.as_str().to_string(),
                        Rule::Not(Wire::new(not.as_str())),
                    ))
                } else if let (Some(value), Some(target)) =
                    (captures.name("value"), captures.name("target"))
                {
                    Ok((
                        target.as_str().to_string(),
                        Rule::Direct(Wire::new(value.as_str())),
                    ))
                } else {
                    Err(TaskError(format!("Incorrect rule: {input}")))
                }
            })
            .unwrap_or_else(|| {
                Err(TaskError(format!("Unable to parse rule: {input}")))
            })
    }

    fn calculate(&self, wires: &mut Wires) -> Result<u16> {
        Ok(match self {
            Rule::And(left, right) => {
                left.calculate(wires)? & right.calculate(wires)?
            }
            Rule::Or(left, right) => {
                left.calculate(wires)? | right.calculate(wires)?
            }
            Rule::LShift(left, right) => {
                left.calculate(wires)? << right.calculate(wires)?
            }
            Rule::RShift(left, right) => {
                left.calculate(wires)? >> right.calculate(wires)?
            }
            Rule::Not(value) => !value.calculate(wires)?,
            Rule::Direct(value) => value.calculate(wires)?,
        })
    }
}

type Wires = std::collections::HashMap<String, Rule>;

fn get_rule(wires: &Wires, name: &str) -> Result<Rule> {
    wires
        .get(name)
        .ok_or_else(|| TaskError(format!("No wire '{name}' found")))
        .map(Clone::clone)
}

pub fn parse_input(input: &str) -> Result<Wires> {
    input.lines().map(Rule::new).collect()
}

pub fn task1(wires: &Wires) -> Result<u16> {
    let mut wires = wires.clone();
    get_rule(&wires, "a")?.calculate(&mut wires)
}

pub fn task2(wires: &Wires) -> Result<u16> {
    let mut cwires = wires.clone();
    let a = get_rule(&cwires, "a")?.calculate(&mut cwires)?;
    let mut cwires = wires.clone();
    cwires.insert("b".to_string(), Rule::Direct(Wire::Value(a)));
    get_rule(&cwires, "a")?.calculate(&mut cwires)
}
