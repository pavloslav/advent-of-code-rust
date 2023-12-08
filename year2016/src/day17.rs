use std::collections::HashSet;

pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input.trim())
}

const DIRECTIONS: [(i32, i32, char); 4] = [(0, -1, 'U'), (0, 1, 'D'), (-1, 0, 'L'), (1, 0, 'R')];

struct Traveler {
    passcode: String,
    locations: HashSet<(i32, i32, String)>,
    solutions: Vec<String>,
}

impl Traveler {
    fn new(passcode: &str) -> Traveler {
        Traveler {
            passcode: passcode.to_string(),
            locations: HashSet::from([(0, 0, String::new())]),
            solutions: Vec::new(),
        }
    }
    fn step(&mut self) {
        self.solutions.clear();
        let mut new = HashSet::new();
        for (x, y, path) in &self.locations {
            let mut hasher = common::Md5Hasher::new_from_str(&self.passcode);
            hasher.add_str(path);
            let dirs = hasher.as_str().into_bytes();
            for (&door, (dx, dy, step)) in dirs.iter().zip(DIRECTIONS.iter()) {
                let nx = x + dx;
                let ny = y + dy;
                if door > b'a' && (0..4).contains(&nx) && (0..4).contains(&ny) {
                    let npath = format!("{path}{step}");
                    if (nx, ny) == (3, 3) {
                        self.solutions.push(npath);
                    } else {
                        new.insert((nx, ny, npath));
                    }
                }
            }
        }
        self.locations = new;
    }
}

pub fn task1(input: &str) -> anyhow::Result<String> {
    let mut traveler = Traveler::new(input);
    while traveler.solutions.is_empty() && !traveler.locations.is_empty() {
        traveler.step();
    }
    if traveler.solutions.len() != 1 {
        Err(anyhow::anyhow!(
            "Found {} solutions!",
            traveler.solutions.len()
        ))
    } else {
        Ok(traveler.solutions[0].clone())
    }
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    let mut traveler = Traveler::new(input);
    let mut best = 0;
    while !traveler.locations.is_empty() {
        traveler.step();
        if let Some(l) = traveler.solutions.first() {
            best = best.max(l.len());
        }
    }
    Ok(best)
}
