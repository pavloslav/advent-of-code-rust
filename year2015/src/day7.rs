use crate::*;

#[derive(Clone, Debug)]
pub enum Wire {
    Name(String),
    Value(u16),
}

impl<'a> prse::Parse<'a> for Wire {
    fn from_str(s: &'a str) -> Result<Self, prse::ParseError> {
        Ok(match s.parse() {
            Ok(value) => Wire::Value(value),
            Err(_) => Wire::Name(s.to_string()),
        })
    }
}

impl Wire {
    fn calculate(&self, wires: &mut Wires) -> AocResult<u16> {
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

impl<'a> prse::Parse<'a> for Rule {
    fn from_str(s: &str) -> Result<Self, prse::ParseError> {
        Ok(
            if let Ok((left, op, right)) = prse::try_parse!(s, "{} {} {}") {
                match op {
                    "AND" => Rule::And(left, right),
                    "OR" => Rule::Or(left, right),
                    "LSHIFT" => Rule::LShift(left, right),
                    "RSHIFT" => Rule::RShift(left, right),
                    other => {
                        return Err(prse::ParseError::new(format!(
                            "Incorrect binary operation: {other}"
                        )))
                    }
                }
            } else if let Ok(operand) = prse::try_parse!(s, "NOT {}") {
                Rule::Not(operand)
            } else {
                Rule::Direct(prse::try_parse!(s, "{}")?)
            },
        )
    }
}

impl Rule {
    fn calculate(&self, wires: &mut Wires) -> AocResult<u16> {
        Ok(match self {
            Rule::And(left, right) => left.calculate(wires)? & right.calculate(wires)?,
            Rule::Or(left, right) => left.calculate(wires)? | right.calculate(wires)?,
            Rule::LShift(left, right) => left.calculate(wires)? << right.calculate(wires)?,
            Rule::RShift(left, right) => left.calculate(wires)? >> right.calculate(wires)?,
            Rule::Not(value) => !value.calculate(wires)?,
            Rule::Direct(value) => value.calculate(wires)?,
        })
    }
}

type Wires = std::collections::HashMap<String, Rule>;

fn get_rule(wires: &Wires, name: &str) -> AocResult<Rule> {
    wires
        .get(name)
        .ok_or_else(|| aoc_error!("No wire '{name}' found"))
        .map(Clone::clone)
}

pub fn parse_input(input: &str) -> AocResult<Wires> {
    input
        .lines()
        .map(|l| {
            let (target, rule) = prse::try_parse!(l, "{1} -> {0}")?;
            Ok((target, rule))
        })
        .collect()
}

pub fn task1(wires: &Wires) -> AocResult<u16> {
    let mut wires = wires.clone();
    get_rule(&wires, "a")?.calculate(&mut wires)
}

pub fn task2(wires: &Wires) -> AocResult<u16> {
    let mut cwires = wires.clone();
    let a = get_rule(&cwires, "a")?.calculate(&mut cwires)?;
    let mut cwires = wires.clone();
    cwires.insert("b".to_string(), Rule::Direct(Wire::Value(a)));
    get_rule(&cwires, "a")?.calculate(&mut cwires)
}
