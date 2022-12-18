pub fn parse_input(input: &str) -> &str {
    input.trim()
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

struct Tetris {
    tube: Vec<Vec<u8>>,
    figure: &'static [&'static [u8]],
    x: i32,
    y: i32,
    count: usize,
}

impl Tetris {
    fn new() -> Tetris {
        Tetris {
            tube: vec![EMPTY_LINE.to_vec(); 4],
            figure: FIGURES[0],
            x: 2,
            y: 3,
            count: 0,
        }
    }
    fn add_figure(&mut self) {
        self.count += 1;
        self.figure = FIGURES[self.count % 5];
        self.x = 2;
        let free_lines = self.tube.len() - self.tower_height();
        if free_lines < 3 + self.figure.len() {
            self.tube.resize(
                self.tube.len() + 3 + self.figure.len() - free_lines,
                EMPTY_LINE.to_vec(),
            );
        }
        self.y = (self.tower_height() + 3).try_into().unwrap();
    }
    fn move_figure(&mut self, dir: u8) {
        let x = self.x
            + match dir {
                b'>' => 1,
                b'<' => -1,
                _ => unimplemented!("Invalid move {}", dir),
            };
        if (0..=(TUBE_WIDTH - self.figure[0].len()) as i32).contains(&x)
            && self.fits(x as usize, self.y as usize)
        {
            self.x = x;
        }
        if self.y > 0 && self.fits(self.x as usize, (self.y - 1) as usize) {
            self.y -= 1;
        } else {
            self.fix_figure();
            self.add_figure();
        }
    }
    fn fits(&self, x: usize, y: usize) -> bool {
        self.figure.iter().enumerate().all(|(fy, fline)| {
            fline.iter().enumerate().all(|(fx, &fchar)| {
                fchar != b'#' || self.tube[y + fy][x + fx] != b'#'
            })
        })
    }
    fn fix_figure(&mut self) {
        for f_y in 0..self.figure.len() {
            for f_x in 0..self.figure[f_y].len() {
                if self.figure[f_y][f_x] == b'#' {
                    self.tube[self.y as usize + f_y][self.x as usize + f_x] =
                        b'#';
                }
            }
        }
    }
    fn tower_height(&self) -> usize {
        self.tube.len()
            - self
                .tube
                .iter()
                .rev()
                .take_while(|row| !row.contains(&b'#'))
                .count()
    }
    fn play(&mut self, moves: &str, limit: usize) -> usize {
        for &mov in moves.as_bytes().iter().cycle() {
            self.move_figure(mov);
            if self.count >= limit {
                break;
            }
        }
        self.tower_height()
    }
    fn print_top_n(&self, n: usize) {
        println!("-----------------");
        for i in 0..n.min(self.tube.len()) {
            let line = std::str::from_utf8(&self.tube[self.tube.len() - 1 - i])
                .unwrap();
            println!("{}", line);
        }
    }
}

pub fn task1(input: &str) -> usize {
    let mut tetris = Tetris::new();
    tetris.play(input, 2022)
}

pub fn task2(_input: &str) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str = &">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    #[test]
    fn test_task1() {
        assert_eq!(task1(EXAMPLE), 3068);
    }

    /*#[test]
    fn test_temp() {
        let mut tetris = Tetris::new();
        tetris.play(EXAMPLE, 10);
        tetris.print_top_n(20);
        assert!(false);
    }*/
}
