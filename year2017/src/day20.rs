use crate::*;

type Coord = [i64; 3];

#[derive(Clone, Copy)]
pub struct Particle([Coord; 3]);

const POS: usize = 0;
const VEL: usize = 1;
const ACC: usize = 2;

pub fn parse_input(input: &str) -> Result<Vec<Particle>> {
    input
        .lines()
        .map(|line| {
            let (px, py, pz, vx, vy, vz, ax, ay, az) = scan_fmt::scan_fmt!(
                line,
                "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>,",
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64
            )?;
            Ok(Particle([[px, py, pz], [vx, vy, vz], [ax, ay, az]]))
        })
        .collect()
}

pub fn task1(input: &[Particle]) -> Result<usize> {
    Ok(input
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.0[ACC].iter().map(|i| i.abs()).sum::<i64>()))
        .min_by_key(|&(_, s)| s)
        .ok_or_else(|| aoc_error!("Input is empty!"))?
        .0)
}

pub fn task2(input: &[Particle]) -> Result<usize> {
    let mut survives = 0;
    let mut particles = input.to_vec();

    for _time in 0.. {
        let mut points = std::collections::HashMap::new();
        let mut collitions = std::collections::HashSet::new();

        let mut extremums = [particles[0]; 2];

        for (idx, pt) in particles.iter().enumerate() {
            if let Some(&present) = points.get(&pt.0[POS]) {
                collitions.insert(present);
                collitions.insert(idx);
            } else {
                points.insert(&pt.0[POS], idx);
            }
            for coord in 0..3 {
                for kind in POS..=ACC {
                    extremums[0].0[kind][coord] =
                        extremums[0].0[kind][coord].max(pt.0[kind][coord]);
                    extremums[1].0[kind][coord] =
                        extremums[1].0[kind][coord].min(pt.0[kind][coord]);
                }
            }
        }
        particles = particles
            .iter()
            .enumerate()
            .filter_map(|(idx, pt)| {
                if collitions.contains(&idx) {
                    None
                } else if (0..6).any(|coord| {
                    (POS..=ACC).all(|kind| {
                        extremums[coord % 2].0[kind][coord / 2]
                            == pt.0[kind][coord / 2]
                    })
                }) {
                    survives += 1;
                    None
                } else {
                    let mut new_pt = Particle([[0; 3]; 3]);
                    for coord in 0..3 {
                        new_pt.0[POS][coord] = pt.0[POS][coord]
                            + pt.0[VEL][coord]
                            + pt.0[ACC][coord];
                        new_pt.0[VEL][coord] =
                            pt.0[VEL][coord] + pt.0[ACC][coord];
                        new_pt.0[ACC][coord] = pt.0[ACC][coord];
                    }
                    Some(new_pt)
                }
            })
            .collect();
        if particles.is_empty() {
            return Ok(survives);
        }
    }
    Err(aoc_error!("unreachable"))
}