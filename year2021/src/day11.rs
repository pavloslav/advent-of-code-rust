use anyhow::Context;

#[derive(Clone)]
pub struct Octopuses {
    map: Vec<Vec<u8>>,
    last_flashed: usize,
}

impl Octopuses {
    fn new(input: &str) -> anyhow::Result<Octopuses> {
        Ok(Octopuses {
            map: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            Ok(c.to_digit(10).with_context(|| format!("Wrong digit {c}"))? as u8)
                        })
                        .collect::<anyhow::Result<Vec<u8>>>()
                })
                .collect::<anyhow::Result<_>>()?,
            last_flashed: 0,
        })
    }

    fn step(&mut self) -> usize {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                self.map[y][x] += 1;
            }
        }
        self.last_flashed = 0;
        loop {
            let mut flashed = 0;
            for y in 0..self.map.len() {
                for x in 0..self.map[y].len() {
                    if self.try_flash(y, x) {
                        flashed += 1;
                    }
                }
            }
            if flashed == 0 {
                break;
            } else {
                self.last_flashed += flashed;
            }
        }
        self.last_flashed
    }

    fn try_flash(&mut self, y: usize, x: usize) -> bool {
        use std::cmp::{max, min};
        if self.map[y][x] <= 9 {
            false
        } else {
            self.map[y][x] = 0;
            for dy in max(0, y as i32 - 1) as usize..min(self.map.len(), y + 2) {
                for dx in max(0, x as i32 - 1) as usize..min(self.map[y].len(), x + 2) {
                    if self.map[dy][dx] != 0 {
                        self.map[dy][dx] += 1;
                    }
                }
            }
            true
        }
    }

    fn area(&self) -> usize {
        self.map.len() * self.map[0].len()
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Octopuses> {
    Octopuses::new(input)
}

pub fn task1(octopuses: &Octopuses) -> anyhow::Result<usize> {
    let mut octopuses = octopuses.clone();
    Ok((0..100).map(|_| octopuses.step()).sum())
}

pub fn task2(octopuses: &Octopuses) -> anyhow::Result<usize> {
    let mut octopuses = octopuses.clone();
    let area = octopuses.area();
    Ok((0..).take_while(|_| octopuses.step() != area).count() + 1)
}
