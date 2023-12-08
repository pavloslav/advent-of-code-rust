pub fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    input
        .trim()
        .bytes()
        .map(|c| match c {
            b'<' => Ok(-1),
            b'>' => Ok(1),
            other => anyhow::bail!("Unknown jet direction '{other}'"),
        })
        .collect()
}

const TUBE_WIDTH: usize = 7;

#[rustfmt::skip]
const FIGURES: &[&[&[u8]]] = &[
    &["####".as_bytes()],

    &[".#.".as_bytes(), 
      "###".as_bytes(), 
      ".#.".as_bytes()],

    &["###".as_bytes(),
      "..#".as_bytes(), 
      "..#".as_bytes()],

    &["#".as_bytes(), 
      "#".as_bytes(), 
      "#".as_bytes(), 
      "#".as_bytes()],

    &["##".as_bytes(), 
      "##".as_bytes()],
];

const EMPTY_LINE: &[u8] = ".......".as_bytes();

#[derive(Clone)]
struct Tetris<'commands> {
    tube: std::collections::VecDeque<Vec<u8>>,
    figure_idx: usize,
    commands: &'commands [i32],
    commands_idx: usize,
    skipped: usize,
}

impl<'commands> Tetris<'commands> {
    fn new(commands: &'commands [i32]) -> Tetris {
        Tetris {
            tube: std::collections::VecDeque::new(),
            figure_idx: 0,
            commands,
            commands_idx: 0,
            skipped: 0,
        }
    }
    fn play(&mut self) {
        let mut x = 2;
        let mut y = -4;
        loop {
            let next_x = x + self.commands[self.commands_idx];
            self.commands_idx += 1;
            if self.commands_idx == self.commands.len() {
                self.commands_idx = 0;
            }
            if self.fits(next_x, y) {
                x = next_x;
            }
            let next_y = y + 1;
            if self.fits(x, next_y) {
                y = next_y;
            } else {
                break;
            }
        }
        self.fix_figure(x, y);
        self.figure_idx += 1;
        if self.figure_idx == FIGURES.len() {
            self.figure_idx = 0;
        }
    }
    fn fits(&self, x: i32, y: i32) -> bool {
        if x < 0 || x as usize + FIGURES[self.figure_idx][0].len() > TUBE_WIDTH {
            return false;
        }
        if y < 0 {
            return true;
        }
        if y >= self.tube.len() as i32 {
            return false;
        }
        FIGURES[self.figure_idx]
            .iter()
            .enumerate()
            .all(|(fy, fline)| {
                fline.iter().enumerate().all(|(fx, &fchar)| {
                    fchar != b'#'
                        || y - (fy as i32) < 0
                        || self.tube[(y - fy as i32) as usize][(x + fx as i32) as usize] != b'#'
                })
            })
    }
    fn fix_figure(&mut self, x: i32, y: i32) {
        for f_y in 0..FIGURES[self.figure_idx].len() {
            if (y - f_y as i32) < 0 || self.tube.is_empty() {
                self.tube.push_front(EMPTY_LINE.to_vec());
            }
            for f_x in 0..FIGURES[self.figure_idx][f_y].len() {
                if FIGURES[self.figure_idx][f_y][f_x] == b'#' {
                    self.tube[(y - f_y as i32).max(0) as usize][x as usize + f_x] = b'#';
                }
            }
        }
    }
    fn tower_height(&self) -> usize {
        self.skipped + self.tube.len()
    }
    fn _print_top_n(&self, n: usize) {
        println!("-----------------");
        for line in self.tube.iter().take(n) {
            println!("{}", std::str::from_utf8(line).unwrap());
        }
    }
    fn gaps(&self) -> impl Iterator<Item = usize> + '_ {
        (0..TUBE_WIDTH).map(|i| {
            self.tube
                .iter()
                .map(|line| line[i])
                .take_while(|&x| x != b'#')
                .count()
        })
    }
}

impl PartialEq for Tetris<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.figure_idx == other.figure_idx
            && self.commands_idx == other.commands_idx
            && self.gaps().cmp(other.gaps()) == std::cmp::Ordering::Equal
    }
}

const STEPS: usize = 2022;

pub fn task1(input: &[i32]) -> anyhow::Result<usize> {
    let mut tetris = Tetris::new(input);
    for _ in 0..STEPS {
        tetris.play();
    }
    Ok(tetris.tower_height())
}

use common::floyd_hare_tortoise;

const ELEPHANT_STEPS: usize = 1_000_000_000_000;

pub fn task2(input: &[i32]) -> anyhow::Result<usize> {
    let (lambda, mu) = floyd_hare_tortoise(|| Tetris::new(input), |t| t.play());

    let mut tetris = Tetris::new(input);
    for _ in 0..mu {
        tetris.play();
    }
    let start_height = tetris.tower_height();
    for _ in 0..lambda {
        tetris.play();
    }
    let period_height = tetris.tower_height() - start_height;
    let remains = (ELEPHANT_STEPS - mu) % lambda;
    for _ in 0..remains {
        tetris.play();
    }
    let remains_height = tetris.tower_height() - start_height - period_height;
    Ok(start_height + (ELEPHANT_STEPS - mu - remains) / lambda * period_height + remains_height)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(EXAMPLE).unwrap()).unwrap(), 3068);
    }

    /*#[test]
    fn test_temp() {
        let mut tetris = Tetris::new(EXAMPLE);
        for _ in 0..40 {
            tetris.play();
            //tetris.print_top_n(20);
        }

        assert!(false);
    }*/
}
