use anyhow::Context;

#[derive(prse::Parse, Clone, Copy, Hash, PartialEq, Eq)]
#[prse = "<{:,:3}>"]
struct Coord([i64; 3]);

impl std::ops::Index<usize> for Coord {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Clone, Copy)]
pub struct Particle([Coord; 3]);

const POS: usize = 0;
const VEL: usize = 1;
const ACC: usize = 2;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Particle>> {
    input
        .lines()
        .map(|line| {
            let (p, v, a) = prse::try_parse!(line, "p={}, v={}, a={}")?;
            Ok(Particle([p, v, a]))
        })
        .collect()
}

pub fn task1(input: &[Particle]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.0[ACC].0.iter().map(|i| i.abs()).sum::<i64>()))
        .min_by_key(|&(_, s)| s)
        .context("Input is empty!")?
        .0)
}

pub fn task2(input: &[Particle]) -> anyhow::Result<usize> {
    let mut survives = 0;
    let mut particles = input.to_vec();

    for _time in 0.. {
        let mut points = std::collections::HashMap::new();
        let mut collitions = std::collections::HashSet::new();

        let mut extremums = [particles[0].0; 2];

        for (idx, pt) in particles.iter().enumerate() {
            if let Some(&present) = points.get(&pt.0[POS]) {
                collitions.insert(present);
                collitions.insert(idx);
            } else {
                points.insert(&pt.0[POS], idx);
            }
            for coord in 0..3 {
                for kind in POS..=ACC {
                    extremums[0][kind].0[coord] = extremums[0][kind][coord].max(pt.0[kind][coord]);
                    extremums[1][kind].0[coord] = extremums[1][kind][coord].min(pt.0[kind][coord]);
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
                    (POS..=ACC)
                        .all(|kind| extremums[coord % 2][kind][coord / 2] == pt.0[kind][coord / 2])
                }) {
                    survives += 1;
                    None
                } else {
                    let mut new_pt = Particle([Coord([0; 3]); 3]);
                    for coord in 0..3 {
                        new_pt.0[POS].0[coord] =
                            pt.0[POS][coord] + pt.0[VEL][coord] + pt.0[ACC][coord];
                        new_pt.0[VEL].0[coord] = pt.0[VEL][coord] + pt.0[ACC][coord];
                        new_pt.0[ACC].0[coord] = pt.0[ACC][coord];
                    }
                    Some(new_pt)
                }
            })
            .collect();
        if particles.is_empty() {
            return Ok(survives);
        }
    }
    Err(anyhow::anyhow!("unreachable"))
}
