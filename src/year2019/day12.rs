use super::super::common::floyd_hare_tortoise::floyd_hare_tortoise_with_cmp;
use crate::*;

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
    fn apply_gravity(&mut self, other: &Moon, coord: usize) {
        self.vel[coord] += match self.pos[coord].cmp(&other.pos[coord]) {
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
        }
    }
    fn apply_velocity(&mut self, coord: usize) {
        self.pos[coord] += self.vel[coord];
    }
    fn energy(&self) -> isize {
        Moon::part_energy(&self.pos) * Moon::part_energy(&self.vel)
    }
    fn part_energy(par: &[isize; 3]) -> isize {
        par.iter().map(|x| x.abs()).sum()
    }
}

pub fn parse_input(input: &str) -> Result<Vec<[isize; 3]>> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = scan_fmt::scan_fmt!(
                line,
                "<x={}, y={}, z={}>",
                isize,
                isize,
                isize
            )?;
            Ok([x, y, z])
        })
        .collect()
}

fn step_model(moons: &mut Vec<Moon>) {
    for coord in 0..3 {
        step_model_coord(moons, coord)
    }
}

fn step_model_coord(moons: &mut Vec<Moon>, coord: usize) {
    for i in 0..moons.len() {
        let mut moon = moons[i].clone();
        for (j, other) in moons.iter().enumerate() {
            if i != j {
                moon.apply_gravity(other, coord);
            }
        }
        moons[i] = moon;
    }
    for moon in moons {
        moon.apply_velocity(coord);
    }
}

pub fn task1(input: &[[isize; 3]]) -> Result<isize> {
    let mut moons: Vec<Moon> = input.iter().map(Moon::new).collect();
    for _ in 0..1000 {
        step_model(&mut moons);
    }
    Ok(moons.iter().map(Moon::energy).sum())
}

pub fn task2(input: &[[isize; 3]]) -> Result<usize> {
    Ok((0..3)
        .map(|i| {
            let (lambda, mu) = floyd_hare_tortoise_with_cmp(
                /*gen*/
                || input.iter().map(Moon::new).collect::<Vec<Moon>>(),
                /*step*/ |moons| step_model_coord(moons, i),
                /* eq */
                |hare, tortoise| {
                    hare.iter().zip(tortoise.iter()).all(|(h, t)| {
                        h.pos[i] == t.pos[i] && h.vel[i] == t.vel[i]
                    })
                },
            );
            mu + lambda
        })
        .fold(1, common::lcm))
}
