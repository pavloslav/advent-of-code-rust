use super::super::common::Error::TaskError;
use super::super::common::Result;

#[derive(Copy, Clone)]
pub enum Target {
    Bot(usize),
    Output(usize),
}

#[derive(Clone)]
pub struct Robot {
    hands: Vec<usize>,
    target_hi: Option<Target>,
    target_lo: Option<Target>,
}

use std::collections::HashMap;
type Robots = HashMap<usize, Robot>;
type Output = HashMap<usize, usize>;

impl Target {
    fn new(typ: &str, target: usize) -> Result<Target> {
        match typ {
            "bot" => Ok(Target::Bot(target)),
            "output" => Ok(Target::Output(target)),
            other => Err(TaskError(format!("Unknown target type '{other}'"))),
        }
    }
    fn give(&self, value: usize, bots: &mut Robots, output: &mut Output) {
        match &self {
            Target::Bot(tgt) => bots
                .entry(*tgt)
                .or_insert_with(Robot::new)
                .hands
                .push(value),
            Target::Output(tgt) => {
                output.insert(*tgt, value);
            }
        }
    }
}

pub fn parse_input(input: &str) -> Result<Robots> {
    let mut robots = HashMap::new();
    for line in input.lines() {
        if let Ok((value, bot)) =
            scan_fmt::scan_fmt!(line, "value {d} goes to bot {d}", usize, usize)
        {
            robots
                .entry(bot)
                .or_insert_with(Robot::new)
                .hands
                .push(value);
        } else {
            let (bot, type_lo, tgt_lo, type_hi, tgt_hi) = scan_fmt::scan_fmt!(
                line,
                "bot {} gives low to {} {} and high to {} {}",
                usize,
                String,
                usize,
                String,
                usize
            )?;
            robots.entry(bot).or_insert_with(Robot::new).target_lo =
                Some(Target::new(&type_lo, tgt_lo)?);
            robots.entry(bot).or_insert_with(Robot::new).target_hi =
                Some(Target::new(&type_hi, tgt_hi)?);
        }
    }
    Ok(robots)
}

impl Robot {
    fn new() -> Self {
        Robot {
            hands: Vec::new(),
            target_hi: None,
            target_lo: None,
        }
    }

    fn can_process(&self, bots: &Robots) -> bool {
        if self.hands.len() != 2
            || self.target_lo.is_none()
            || self.target_hi.is_none()
        {
            return false;
        }
        if let Some(Target::Bot(tgt)) = self.target_lo {
            if bots[&tgt].hands.len() > 1 {
                return false;
            }
        }
        if let Some(Target::Bot(tgt)) = self.target_hi {
            if bots[&tgt].hands.len() > 1 {
                return false;
            }
        }
        true
    }

    fn looking_for(&self) -> bool {
        self.hands.len() == 2
            && self.hands.iter().min() == Some(&17)
            && self.hands.iter().max() == Some(&61)
    }

    fn process(
        &mut self,
        bots: &mut Robots,
        output: &mut Output,
    ) -> Result<()> {
        self.target_lo
            .as_ref()
            .ok_or_else(|| TaskError("Failed to get low target".to_string()))?
            .give(
                *self.hands.iter().min().ok_or_else(|| {
                    TaskError("Hands can't be empty!".to_string())
                })?,
                bots,
                output,
            );
        self.target_hi
            .as_ref()
            .ok_or_else(|| TaskError("Failed to get hi target".to_string()))?
            .give(
                *self.hands.iter().max().ok_or_else(|| {
                    TaskError("Hands can't be empty!".to_string())
                })?,
                bots,
                output,
            );
        self.hands.clear();
        Ok(())
    }
}

pub fn task1(robots: &Robots) -> Result<usize> {
    let mut robots = robots.clone();
    let mut output = Output::new();
    let mut changed = true;
    while changed {
        changed = false;
        for &i in robots.keys() {
            if robots[&i].looking_for() {
                return Ok(i);
            }
            if robots[&i].can_process(&robots) {
                let mut robot = robots[&i].clone();
                robot.process(&mut robots, &mut output)?;
                robots.insert(i, robot);
                changed = true;
                break;
            }
        }
    }
    Err(TaskError("Not found".to_string()))
}

pub fn task2(robots: &Robots) -> Result<usize> {
    let mut robots = robots.clone();
    let mut output = Output::new();
    let mut changed = true;
    while changed {
        changed = false;
        for &i in robots.keys() {
            if robots[&i].can_process(&robots) {
                let mut robot = robots[&i].clone();
                robot.process(&mut robots, &mut output)?;
                robots.insert(i, robot);
                changed = true;
                break;
            }
        }
    }
    Ok(output[&0] * output[&1] * output[&2])
}
