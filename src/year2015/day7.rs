use crate::*;

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
    fn new(s: &str) -> Result<(String, Rule)> {
        if let Ok((left, op, right, target)) = scan_fmt::scan_fmt!(
            s,
            "{} {/AND|OR|LSHIFT|RSHIFT/} {} -> {}",
            String,
            String,
            String,
            String
        ) {
            let left = Wire::new(&left);
            let right = Wire::new(&right);
            Ok((
                target,
                match op.as_str() {
                    "AND" => Rule::And(left, right),
                    "OR" => Rule::Or(left, right),
                    "LSHIFT" => Rule::LShift(left, right),
                    "RSHIFT" => Rule::RShift(left, right),
                    other => {
                        return Err(task_error!(
                            "Incorrect binary operation: {other}"
                        ))
                    }
                },
            ))
        } else if let Ok((operand, target)) =
            scan_fmt::scan_fmt!(s, "NOT {} -> {}", String, String)
        {
            Ok((target, Rule::Not(Wire::new(&operand))))
        } else {
            let (wire, target) =
                scan_fmt::scan_fmt!(s, "{} -> {}", String, String)?;
            Ok((target, Rule::Direct(Wire::new(&wire))))
        }
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
        .ok_or_else(|| task_error!("No wire '{name}' found"))
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
