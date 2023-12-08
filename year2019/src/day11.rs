use super::computer::Computer;

use std::collections::HashMap;

struct Robot {
    brain: Computer,
    x: i32,
    y: i32,
    direction: u8, /*0 - up, 1 - right etc.*/
    painted: HashMap<(i32, i32), u8>,
}

impl Robot {
    fn new(code: &[isize]) -> Robot {
        Robot {
            brain: Computer::new(code),
            x: 0,
            y: 0,
            direction: 0,
            painted: HashMap::new(),
        }
    }
    fn turn(&mut self, dir: u8) {
        self.direction = (self.direction + 3 - 2 * dir) % 4; /* MAAAGIC */
    }
    fn walk(&mut self) {
        match self.direction {
            0 => self.y += 1,
            1 => self.x += 1,
            2 => self.y -= 1,
            3 => self.x -= 1,
            _ => unreachable!(),
        }
    }
    fn paint(&mut self, color: u8) {
        self.painted.insert((self.x, self.y), color);
    }
    fn work(&mut self) -> anyhow::Result<()> {
        while !self.brain.is_halted() {
            self.brain
                .write(*self.painted.get(&(self.x, self.y)).unwrap_or(&0) as isize);
            self.brain.run()?;
            let color = self.brain.read().unwrap() as u8;
            self.paint(color);
            let dir = self.brain.read().unwrap() as u8;
            self.turn(dir);
            self.walk();
        }
        Ok(())
    }
    fn painted_string(&self) -> String {
        let mut left = 0;
        let mut right = 0;
        let mut top = 0;
        let mut bottom = 0;
        for &(x, y) in self.painted.keys() {
            right = right.max(x);
            left = left.min(x);
            top = top.max(y);
            bottom = bottom.min(y);
        }
        let width = (right - left + 2) as usize;
        let height = (top - bottom + 1) as usize;
        let mut hull: Vec<u8> = vec![b' '; height * width];
        for i in 0..height {
            hull[width * (i + 1) - 1] = b'\n';
        }
        for (&(x, y), &color) in self.painted.iter() {
            if color == 1 {
                hull[width * (top - y) as usize + (x - left) as usize] = b'#';
            }
        }
        std::str::from_utf8(&hull).unwrap().to_string()
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<isize>> {
    Computer::prepare_code(input)
}

pub fn task1(code: &[isize]) -> anyhow::Result<usize> {
    let mut robot = Robot::new(code);
    robot.work()?;
    Ok(robot.painted.len())
}

pub fn task2(code: &[isize]) -> anyhow::Result<String> {
    let mut robot = Robot::new(code);
    robot.painted.insert((0, 0), 1);
    robot.work()?;
    Ok(robot.painted_string())
}
