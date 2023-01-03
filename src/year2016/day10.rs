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
    fn new(typ: &str, target: usize) -> Target {
        match typ {
            "bot" => Target::Bot(target),
            "output" => Target::Output(target),
            _ => unimplemented!(),
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

fn parse_value(input: &str) -> Result<(usize, usize), text_io::Error> {
    let (value, bot);
    text_io::try_scan!(input.bytes()=>"value {} goes to bot {}", value, bot);
    Ok((value, bot))
}

fn parse_bot(
    input: &str,
) -> Result<(usize, String, usize, String, usize), text_io::Error> {
    let (bot, type_lo, tgt_lo, type_hi, tgt_hi);
    text_io::try_scan!(input.bytes()=>"bot {} gives low to {} {} and high to {} {}", bot, type_lo, tgt_lo, type_hi, tgt_hi);
    Ok((bot, type_lo, tgt_lo, type_hi, tgt_hi))
}

pub fn parse_input(input: &str) -> Robots {
    let mut robots = HashMap::new();
    for line in input.lines() {
        if let Ok((value, bot)) = parse_value(line) {
            robots
                .entry(bot)
                .or_insert_with(Robot::new)
                .hands
                .push(value);
        } else if let Ok((bot, type_lo, tgt_lo, type_hi, tgt_hi)) =
            parse_bot(line)
        {
            robots.entry(bot).or_insert_with(Robot::new).target_lo =
                Some(Target::new(&type_lo, tgt_lo));
            robots.entry(bot).or_insert_with(Robot::new).target_hi =
                Some(Target::new(&type_hi, tgt_hi));
        }
    }
    robots
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

    fn process(&mut self, bots: &mut Robots, output: &mut Output) {
        self.target_lo.as_ref().unwrap().give(
            *self.hands.iter().min().unwrap(),
            bots,
            output,
        );
        self.target_hi.as_ref().unwrap().give(
            *self.hands.iter().max().unwrap(),
            bots,
            output,
        );
        self.hands.clear();
    }
}

pub fn task1(robots: &Robots) -> usize {
    let mut robots = robots.clone();
    let mut output = Output::new();
    let mut changed = true;
    while changed {
        changed = false;
        for &i in robots.keys() {
            if robots[&i].looking_for() {
                return i;
            }
            if robots[&i].can_process(&robots) {
                let mut robot = robots[&i].clone();
                robot.process(&mut robots, &mut output);
                robots.insert(i, robot);
                changed = true;
                break;
            }
        }
    }
    usize::MAX
}

pub fn task2(robots: &Robots) -> usize {
    let mut robots = robots.clone();
    let mut output = Output::new();
    let mut changed = true;
    while changed {
        changed = false;
        for &i in robots.keys() {
            if robots[&i].can_process(&robots) {
                let mut robot = robots[&i].clone();
                robot.process(&mut robots, &mut output);
                robots.insert(i, robot);
                changed = true;
                break;
            }
        }
    }
    output[&0] * output[&1] * output[&2]
}
