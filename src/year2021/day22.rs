#[derive(Clone)]
pub enum State {
    On,
    Off,
}

#[derive(PartialEq, Clone)]
pub struct Zone {
    limits: [std::ops::RangeInclusive<i32>; 3],
}

enum Intersection {
    Intersects(usize),
    Equals,
    None,
}

impl Zone {
    fn intersects(&self, other: &Zone) -> Intersection {
        if self.limits == other.limits {
            return Intersection::Equals;
        }
        for coordinate in 0..3 {
            if self.limits[coordinate].start() > other.limits[coordinate].end()
                || self.limits[coordinate].end()
                    < other.limits[coordinate].start()
            {
                return Intersection::None;
            }
        }
        for coordinate in 0..3 {
            if self.limits[coordinate].start()
                != other.limits[coordinate].start()
                || self.limits[coordinate].end()
                    != other.limits[coordinate].end()
            {
                return Intersection::Intersects(coordinate);
            }
        }

        unreachable!("Zone::intersects fails");
    }
}

#[derive(Clone)]
pub struct Command {
    state: State,
    zone: Zone,
}

impl Command {
    fn new(input: &str) -> Command {
        lazy_static::lazy_static! {
            //on x=-20..26,y=-36..17,z=-47..7
            static ref INPUT_REGEX: regex::Regex = regex::Regex::new(r"(on|off) x=(-?[\d]+)..(-?[\d]+),y=(-?[\d]+)..(-?[\d]+),z=(-?[\d]+)..(-?[\d]+)").unwrap();
        }
        let captures = INPUT_REGEX.captures(input).unwrap();
        let mut iter = captures.iter().skip(1);
        let mut result = Command {
            state: if iter.next().unwrap().unwrap().as_str() == "on" {
                State::On
            } else {
                State::Off
            },
            zone: Zone {
                limits: [0..=0, 0..=0, 0..=0],
            },
        };
        for coordinate in 0..3 {
            result.zone.limits[coordinate] =
                iter.next().unwrap().unwrap().as_str().parse().unwrap()
                    ..=iter.next().unwrap().unwrap().as_str().parse().unwrap();
        }
        result
    }
}

struct Reactor {
    cubes: Vec<Zone>,
}

impl Reactor {
    fn new() -> Reactor {
        Reactor { cubes: Vec::new() }
    }

    fn add(&mut self, command: &Command) {
        let mut idx = 0;
        while idx < self.cubes.len() {
            match self.cubes[idx].intersects(&command.zone) {
                Intersection::Equals => {
                    if let State::Off = command.state {
                        self.cubes.remove(idx);
                    }
                    return;
                }
                Intersection::Intersects(coordinate) => {
                    match self.cubes[idx].limits[coordinate]
                        .start()
                        .cmp(&command.zone.limits[coordinate].start())
                    {
                        std::cmp::Ordering::Less => {
                            let removed = self.cubes.remove(idx).clone();
                            let mut left = removed.clone();
                            left.limits[coordinate] = *removed.limits
                                [coordinate]
                                .start()
                                ..=*command.zone.limits[coordinate].start() - 1;
                            let mut right = removed;
                            right.limits[coordinate] =
                                *command.zone.limits[coordinate].start()
                                    ..=*right.limits[coordinate].end();
                            self.cubes.push(left);
                            self.cubes.push(right);
                        }
                        std::cmp::Ordering::Greater => {
                            let mut left = command.clone();
                            left.zone.limits[coordinate] = *command.zone.limits
                                [coordinate]
                                .start()
                                ..=*self.cubes[idx].limits[coordinate].start()
                                    - 1;
                            let mut right = command.clone();
                            right.zone.limits[coordinate] =
                                *self.cubes[idx].limits[coordinate].start()
                                    ..=*command.zone.limits[coordinate].end();
                            self.add(&left);
                            self.add(&right);
                            return;
                        }
                        std::cmp::Ordering::Equal => {
                            match self.cubes[idx].limits[coordinate]
                                .end()
                                .cmp(&command.zone.limits[coordinate].end())
                            {
                                std::cmp::Ordering::Greater => {
                                    let removed = self.cubes.remove(idx);
                                    let mut left = removed.clone();
                                    left.limits[coordinate] =
                                        *removed.limits[coordinate].start()
                                            ..=*command.zone.limits[coordinate]
                                                .end();
                                    let mut right = removed.clone();
                                    right.limits[coordinate] =
                                        *command.zone.limits[coordinate].end()
                                            + 1
                                            ..=*removed.limits[coordinate]
                                                .end();
                                    self.cubes.push(right);
                                    self.cubes.push(left);
                                }
                                std::cmp::Ordering::Less => {
                                    let mut left = command.clone();
                                    left.zone.limits[coordinate] = *command
                                        .zone
                                        .limits[coordinate]
                                        .start()
                                        ..=*self.cubes[idx].limits[coordinate]
                                            .end();
                                    let mut right = command.clone();
                                    right.zone.limits[coordinate] =
                                        *self.cubes[idx].limits[coordinate]
                                            .end()
                                            + 1
                                            ..=*command.zone.limits[coordinate]
                                                .end();
                                    self.add(&left);
                                    self.add(&right);
                                    return;
                                }
                                _ => unreachable!(
                                    "Both ranges shoun't be equal!"
                                ),
                            }
                        }
                    }
                }
                _ => idx += 1,
            }
        }
        if let State::On = command.state {
            self.cubes.push(command.zone.clone());
        }
    }

    fn count(&self) -> usize {
        self.cubes
            .iter()
            .map(|zone| {
                zone.limits
                    .iter()
                    .map(|range| (range.end() - range.start() + 1) as usize)
                    .product::<usize>()
            })
            .sum()
    }
}

pub fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|line| Command::new(line)).collect()
}

pub fn task1(commands: &Vec<Command>) -> usize {
    let mut reactor = Reactor::new();
    for command in commands {
        if command
            .zone
            .limits
            .iter()
            .all(|range| *range.start() <= 50 && *range.end() >= -50)
        {
            reactor.add(command);
        }
    }
    reactor.count()
}

pub fn task2(commands: &Vec<Command>) -> usize {
    let mut reactor = Reactor::new();
    for command in commands {
        reactor.add(command);
    }
    reactor.count()
}
