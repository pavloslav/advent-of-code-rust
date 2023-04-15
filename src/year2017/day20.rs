type Coord = [i64; 3];

#[derive(Clone, Copy)]
pub struct Particle {
    pos: Coord,
    vel: Coord,
    acc: Coord,
}

pub fn parse_input(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|line| {
            let mut p = Particle {
                pos: [0; 3],
                vel: [0; 3],
                acc: [0; 3],
            };
            for part in line.split(", ") {
                let (typ, x, y, z) = scan_fmt::scan_fmt!(
                    part,
                    "{}=<{},{},{}>",
                    char,
                    i64,
                    i64,
                    i64
                )
                .unwrap();
                match typ {
                    'p' => {
                        p.pos[0] = x;
                        p.pos[1] = y;
                        p.pos[2] = z;
                    }
                    'v' => {
                        p.vel[0] = x;
                        p.vel[1] = y;
                        p.vel[2] = z;
                    }
                    'a' => {
                        p.acc[0] = x;
                        p.acc[1] = y;
                        p.acc[2] = z;
                    }
                    _ => panic!(),
                }
            }
            p
        })
        .collect()
}

pub fn task1(input: &[Particle]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.acc.iter().map(|i| i.abs()).sum::<i64>()))
        .min_by_key(|&(_, s)| s)
        .unwrap()
        .0
}

pub fn task2(input: &[Particle]) -> usize {
    let mut survives = 0;
    let mut particles = input.to_vec();

    for _time in 0.. {
        let mut points = std::collections::HashMap::new();
        let mut collitions = std::collections::HashSet::new();

        let mut extremums = [particles[0]; 2];

        for (idx, pt) in particles.iter().enumerate() {
            if let Some(&present) = points.get(&pt.pos) {
                collitions.insert(present);
                collitions.insert(idx);
            } else {
                points.insert(&pt.pos, idx);
            }
            for coord in 0..3 {
                extremums[0].pos[coord] =
                    extremums[0].pos[coord].max(pt.pos[coord]);
                extremums[1].pos[coord] =
                    extremums[1].pos[coord].min(pt.pos[coord]);
                extremums[0].vel[coord] =
                    extremums[0].vel[coord].max(pt.vel[coord]);
                extremums[1].vel[coord] =
                    extremums[1].vel[coord].min(pt.vel[coord]);
                extremums[0].acc[coord] =
                    extremums[0].acc[coord].max(pt.acc[coord]);
                extremums[1].acc[coord] =
                    extremums[1].acc[coord].min(pt.acc[coord]);
            }
        }
        particles = particles
            .iter()
            .enumerate()
            .filter_map(|(idx, pt)| {
                if collitions.contains(&idx) {
                    None
                } else if (0..6).any(|coord| {
                    extremums[coord % 2].pos[coord / 2] == pt.pos[coord / 2]
                        && extremums[coord % 2].vel[coord / 2]
                            == pt.vel[coord / 2]
                        && extremums[coord % 2].acc[coord / 2]
                            == pt.acc[coord / 2]
                }) {
                    survives += 1;
                    None
                } else {
                    let mut new_pt = Particle {
                        pos: [0; 3],
                        vel: [0; 3],
                        acc: [0; 3],
                    };
                    for coord in 0..3 {
                        new_pt.pos[coord] =
                            pt.pos[coord] + pt.vel[coord] + pt.acc[coord];
                        new_pt.vel[coord] = pt.vel[coord] + pt.acc[coord];
                        new_pt.acc[coord] = pt.acc[coord];
                    }
                    Some(new_pt)
                }
            })
            .collect();
        if particles.is_empty() {
            return survives;
        }
    }
    unreachable!()
}
