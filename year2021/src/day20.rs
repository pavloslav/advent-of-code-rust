#[derive(Clone)]
struct Algo(Vec<u8>);

#[derive(Clone)]
struct Image(Vec<Vec<u8>>);

pub struct ImageData {
    algo: Algo,
    image: Image,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            for &ch in line {
                write!(f, "{}", if ch == 1 { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn char_to_bit(c: char) -> u8 {
    match c {
        '#' => 1,
        '.' => 0,
        _ => panic!("Wrong char in input: {}", c),
    }
}

pub fn parse_input(input: &str) -> ImageData {
    let mut input = input.lines();
    let algo = Algo(input.next().unwrap().chars().map(char_to_bit).collect());
    let image = Image(
        input
            .skip(1)
            .map(|line| line.chars().map(char_to_bit).collect())
            .collect(),
    );
    ImageData { algo, image }
}

impl Image {
    fn enhance(&mut self, algo: &Algo, step: usize) {
        let height = self.0.len();
        let width = self.0[1].len();
        let outer = algo.get_outer_symbol(step);
        self.0 = (0..height + 2)
            .map(|y| {
                (0..width + 2)
                    .map(|x| {
                        algo.0
                            [self.get_binary(x as i32 - 1, y as i32 - 1, outer)]
                    })
                    .collect()
            })
            .collect();
    }

    fn get_binary(&self, x: i32, y: i32, outer: u8) -> usize {
        let mut binary = 0;
        for i in y - 1..y + 2 {
            for j in x - 1..x + 2 {
                let v = if 0 <= i
                    && i < self.0.len() as i32
                    && 0 <= j
                    && j < self.0[0].len() as i32
                {
                    self.0[i as usize][j as usize]
                } else {
                    outer
                };
                binary = (binary << 1) | v as usize;
            }
        }
        binary
    }

    fn lit_pixels(&self) -> usize {
        self.0
            .iter()
            .flat_map(|line| line.iter().map(|&bit| bit as usize))
            .sum()
    }
}

impl Algo {
    fn get_outer_symbol(&self, step: usize) -> u8 {
        if step == 0 || *self.0.first().unwrap() == 0 {
            0
        } else if *self.0.last().unwrap() == 1 {
            1
        } else {
            (step % 2) as u8
        }
    }
}

pub fn task(input: &ImageData, count: usize) -> usize {
    let mut image = input.image.clone();
    for i in 0..count {
        image.enhance(&input.algo, i);
    }
    image.lit_pixels()
}

pub fn task1(input: &ImageData) -> usize {
    task(input, 2)
}

pub fn task2(input: &ImageData) -> usize {
    task(input, 50)
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1, task2};

    fn get_test_data() -> ImageData {
        let data = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        parse_input(&data)
    }

    #[test]
    fn test_get_binary() {
        let data = get_test_data();
        assert_eq!(data.image.get_binary(2, 2, 0), 34);
    }

    #[test]
    fn test_task1() {
        assert_eq!(task1(&get_test_data()), 35);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&get_test_data()), 3351);
    }
}
