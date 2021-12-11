mod ferry {
    enum ModelKind {
        Close,
        Distant
    }

    const FLOOR    : char = '.';
    const EMPTY    : char = 'L';
    const OCCUPIED : char = '#';
    const CLOSE_OVERCROWD   : usize = 4;
    const DISTANT_OVERCROWD : usize = 5;


    pub struct Model {
        seats: Vec<Vec<char>>,
        kind : ModelKind,
    }

    const  SHIFTS:[(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                     ( 0, -1),          ( 0, 1),
                                     ( 1, -1), ( 1, 0), ( 1, 1)];

    impl Model {
        fn str_to_seats(s: &str) -> Vec<Vec<char>>
        {
            s.lines()
             .map(|line|line.chars()
                            .collect())
             .collect()
        }
        pub fn close(s: &str) -> Model {
            Model { seats: Model::str_to_seats(s), 
                    kind:  ModelKind::Close,
            }
        }
        pub fn distant(s: &str) -> Model {
            Model { seats: Model::str_to_seats(s), 
                    kind : ModelKind::Distant,
            }
        }

        fn rows(&self) -> usize {
            self.seats.len()
        }
        fn cols(&self) -> usize {
            self.seats[0].len()
        }
        fn count_neighbors(&self, row:usize, col:usize) -> usize {
            match self.kind {
                ModelKind::Close => self.count_close_neighbors(row, col),
                ModelKind::Distant => self.count_distant_neighbors(row, col)
            }
        }
        fn count_close_neighbors(&self, row:usize, col:usize) -> usize
        {
            SHIFTS.iter()
                  .map(|(shift_row, shift_col)|(row as i32 + shift_row, col as i32 + shift_col))
                  .filter(|&(row, col)|{ 0<=row && row<self.rows() as i32
                                      && 0<=col && col<self.cols() as i32
                                      && self.seats[row as usize][col as usize] == OCCUPIED })
                  .count()
        }
        fn count_distant_neighbors(&self, row:usize, col:usize) -> usize
        {
            SHIFTS.iter()
                  .map(|(shift_row, shift_col)|
                     (1..).map(|distance|(row as i32 + distance * shift_row, col as i32 + distance * shift_col))
                          .take_while(|&(row, col)| 0<=row && row<self.rows() as i32
                                                 && 0<=col && col<self.cols() as i32
                                                 && self.seats[row as usize][col as usize] != EMPTY)
                          .filter(|&(row, col)|self.seats[row as usize][col as usize] == OCCUPIED)
                          .take(1)
                          .count())
                  .sum()
        }
        fn overcrowd(&self) -> usize
        {
            match self.kind {
                ModelKind::Close   => CLOSE_OVERCROWD,
                ModelKind::Distant => DISTANT_OVERCROWD,
            }
        }
        fn step(&mut self) -> bool {
            let mut changed = false;
            self.seats = 
                (0..self.rows()).map(|row|
                    (0..self.cols()).map(|col|{
                        if self.seats[row][col]==FLOOR {
                            FLOOR
                        } else {
                            let occupied = self.count_neighbors(row, col);
                            //println!("Place {} {} has {} occupied neighbors", row, col, occupied);
                            if occupied == 0 && self.seats[row][col]==EMPTY {
                                changed = true;
                                OCCUPIED
                            } else if occupied>=self.overcrowd() && self.seats[row][col]==OCCUPIED {
                                changed = true;
                                EMPTY
                            } else {
                                self.seats[row][col]
                            }
                        }
                    }
                    ).collect()
                ).collect();
            changed
        }
        fn count(&self, kind: char) -> usize {
            self.seats.iter()
                      .map(|row|
                        row.iter().filter(|&&cell|cell==kind).count()
            ).sum()
        }

        pub fn get_stable_count(&mut self) -> usize {
            while self.step() {
                //println!("{}", self);
            }
            self.count(OCCUPIED)
        }
    }
    impl std::fmt::Display for Model {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Cells:----------------")?;
            for row in self.seats.iter() {
                for ch in row.iter() {
                    write!(f, "{}", ch)?;
                }
                writeln!(f, "")?;
            }
            Ok(())
        }
    }
    #[cfg(test)]
    mod test {
         use super::*;
        
        #[test]
        fn test_neighbors() {
            let model = Model::distant(
".............
.L.L.#.#.#.#.
.............");
            assert_eq!(model.count_neighbors(1,1), 0);
            assert_eq!(model.count_neighbors(1,3), 1);
            let  model = Model::distant(
".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.");
            assert_eq!(model.count_neighbors(3,3), 0);

        }
    }
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s:&str) -> usize {
    let mut model = ferry::Model::close(s);
    model.get_stable_count()
}

pub fn task2(s:&str) -> usize {
    let mut model = ferry::Model::distant(s);
    model.get_stable_count()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_tasks() {
        let input1 = 
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(task1(input1), 37);
        assert_eq!(task2(input1), 26);
    }
}