use crate::*;

#[derive(Debug)]
struct Line {
    dst: usize,
    src: usize,
    len: usize,
}

#[derive(Default, Debug)]
struct Map(Vec<Line>);

impl Map {
    fn transform(&self, input: usize) -> usize {
        self.0
            .iter()
            .find_map(|line| {
                if line.src <= input && input < line.src + line.len {
                    Some(line.dst + input - line.src)
                } else {
                    None
                }
            })
            .unwrap_or(input)
    }
}

#[derive(Default)]
pub struct Almonac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almonac {
    fn add_line(&mut self, line: Line) {
        if let Some(map) = self.maps.last_mut() {
            map.0.push(line);
        } else {
            self.maps.push(Map(vec![line]));
        }
    }
    fn new_with_seeds(seeds: Vec<usize>) -> Self {
        Self {
            seeds,
            ..Self::default()
        }
    }
}

pub fn parse_input(input: &str) -> AocResult<Almonac> {
    let mut lines = input.lines();
    let mut almonac = Almonac::new_with_seeds(prse::try_parse!(
        lines
            .next()
            .ok_or_else(|| aoc_error!("Failed to parse seeds"))?,
        "seeds: {: :}"
    )?);
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Ok((dst, src, len)) = prse::try_parse!(line, "{} {} {}") {
            almonac.add_line(Line { dst, src, len });
        } else {
            let (_from, _to): (&str, &str) = prse::try_parse!(line, "{}-to-{} map:")?;
            almonac.maps.push(Map::default());
        }
    }
    Ok(almonac)
}

pub fn task1(input: &Almonac) -> AocResult<usize> {
    let mut values = input.seeds.clone();
    for map in &input.maps {
        values = values.iter().map(|&v| map.transform(v)).collect();
    }
    values
        .iter()
        .min()
        .copied()
        .ok_or_else(|| aoc_error!("Empty values vec!"))
}

pub fn task2(input: &Almonac) -> AocResult<usize> {
    let mut values: Vec<_> = input
        .seeds
        .chunks(2)
        .map(|sub| {
            if let &[a, b] = sub {
                Ok((a, b))
            } else {
                Err(aoc_error!("Incorrect number of seeds"))
            }
        })
        .collect::<AocResult<_>>()?;
    for map in &input.maps {
        let mut new_values = vec![]; //std::collections::HashSet::new();
        while let Some((mut src, mut len)) = values.pop() {
            for line in &map.0 {
                if src < line.src && line.src < src + len {
                    let new_len = line.src - src;
                    values.push((line.src, len - new_len));
                    len = new_len;
                }
                if line.src <= src && src < line.src + line.len {
                    if src + len > line.src + line.len {
                        let new_len = src + len - line.src - line.len;
                        values.push((line.src + line.len, new_len));
                        len -= new_len;
                    }
                    src = line.dst + src - line.src;
                    break;
                }
            }
            new_values.push((src, len)); //.insert((src, len));
        }
        values = new_values; //.iter().copied().collect();
    }
    values
        .iter()
        .map(|(a, _)| a)
        .min()
        .copied()
        .ok_or_else(|| aoc_error!("Empty values vec!"))
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_task1() {
        let almonac = parse_input(INPUT).unwrap();
        assert_eq!(task1(&almonac).unwrap(), 35);
    }
    #[test]
    fn test_task2() {
        let almonac = parse_input(INPUT).unwrap();
        assert_eq!(task2(&almonac).unwrap(), 46);
    }
}
