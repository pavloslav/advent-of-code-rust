#[derive(Clone, PartialEq, Eq)]
pub struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    fn new(pos: &[isize; 3]) -> Moon {
        Moon {
            pos: *pos,
            vel: [0; 3],
        }
    }
    fn apply_gravity(&mut self, other: &Moon) {
        for i in 0..3 {
            self.vel[i] += match self.pos[i].cmp(&other.pos[i]) {
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
            }
        }
    }
    fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }
    fn energy(&self) -> isize {
        Moon::part_energy(&self.pos) * Moon::part_energy(&self.vel)
    }
    fn part_energy(par: &[isize; 3]) -> isize {
        par.iter().map(|x| x.abs()).sum()
    }
}

pub fn parse_input(input: &str) -> Vec<[isize; 3]> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = scan_fmt::scan_fmt!(
                line,
                "<x={}, y={}, z={}>",
                isize,
                isize,
                isize
            )
            .unwrap();
            [x, y, z]
        })
        .collect()
}

fn step_model(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        let mut moon = moons[i].clone();
        for (j, other) in moons.iter().enumerate() {
            if i != j {
                moon.apply_gravity(other);
            }
        }
        moons[i] = moon;
    }
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

pub fn task1(input: &[[isize; 3]]) -> isize {
    let mut moons: Vec<Moon> = input.iter().map(Moon::new).collect();
    for _ in 0..1000 {
        step_model(&mut moons);
    }
    moons.iter().map(Moon::energy).sum()
}

use num::integer::lcm;

pub fn task2(input: &[[isize; 3]]) -> usize {
    let mut periods = [0usize; 3];
    for (i, period) in periods.iter_mut().enumerate() {
        let mut tortoise: Vec<Moon> = input.iter().map(Moon::new).collect();
        let mut hare = tortoise.clone();
        step_model(&mut tortoise);
        step_model(&mut hare);
        step_model(&mut hare);
        while tortoise
            .iter()
            .zip(hare.iter())
            .any(|(t, h)| t.pos[i] != h.pos[i] || t.vel[i] != h.vel[i])
        {
            step_model(&mut tortoise);
            step_model(&mut hare);
            step_model(&mut hare);
        }
        tortoise = input.iter().map(Moon::new).collect();
        let mut mu = 0usize;
        while tortoise
            .iter()
            .zip(hare.iter())
            .any(|(t, h)| t.pos[i] != h.pos[i] || t.vel[i] != h.vel[i])
        {
            step_model(&mut tortoise);
            step_model(&mut hare);
            mu += 1;
        }
        let mut lambda = 1usize;
        step_model(&mut hare);
        while tortoise
            .iter()
            .zip(hare.iter())
            .any(|(t, h)| t.pos[i] != h.pos[i] || t.vel[i] != h.vel[i])
        {
            step_model(&mut hare);
            lambda += 1;
        }
        *period = mu + lambda;
    }
    lcm(lcm(periods[0], periods[1]), periods[2])
}
