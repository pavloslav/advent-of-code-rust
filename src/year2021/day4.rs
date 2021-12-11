const BINGO_SIZE :usize = 5;

struct Board ([[usize; BINGO_SIZE];BINGO_SIZE] );

impl Board {
    fn position(&self, value: usize) -> Option<(usize, usize)> {
        for row in 0..BINGO_SIZE {
            for col in 0..BINGO_SIZE {
                if self.0[row][col] == value {
                    return Some((row, col));
                }
            }
        }
        None
    }
}

pub struct BingoSettings
{
    calls: Vec<usize>,
    boards: Vec<Board>,
}

impl BingoSettings
{
    fn from_str(s: &str) -> BingoSettings {
        let mut s = s.lines();
        let calls = s.next()
                     .unwrap()
                     .split(",")
                     .map(|s|s.parse().unwrap())
                     .collect();
        let mut boards = Vec::new();
        let mut idx = 0;
        for line in s {
            if line.is_empty() {
                boards.push(Board([[0, 0, 0, 0, 0, ],
                                   [0, 0, 0, 0, 0, ],
                                   [0, 0, 0, 0, 0, ],
                                   [0, 0, 0, 0, 0, ],
                                   [0, 0, 0, 0, 0, ],]));
                idx = 0;
            } else {
                let mut board = boards.last_mut().unwrap();
                for (i, n) in line.split_whitespace().enumerate() {
                    board.0[idx][i] = n.parse().unwrap();
                }
                idx = (idx+1) % 5;
            }
        }
        BingoSettings {
            calls,
            boards,
        }
    }
}

type StrikeBoard = [[bool; BINGO_SIZE];BINGO_SIZE];

struct Bingo<'a>
{
    settings: &'a BingoSettings,
    winners: Vec<(usize, usize)>,
    striked: Vec<StrikeBoard>,
}

impl Bingo<'_>
{
    fn new(settings: &BingoSettings) -> Bingo
    {
        Bingo {
            settings,
            winners: Vec::new(),
            striked: std::iter::repeat([[false, false, false, false, false, ],
                                        [false, false, false, false, false, ],
                                        [false, false, false, false, false, ],
                                        [false, false, false, false, false, ],
                                        [false, false, false, false, false, ],])
            .take(settings.boards.len())
            .collect(),
        }
    }

    fn task(&mut self, nwinner:usize) -> usize {
        for &call in &self.settings.calls {
            self.strikeout(call);
            if self.winners.len() == nwinner {
                break;
            }
        }
        self.last_winner_score()
    }

    fn strikeout(&mut self, call: usize) {
        for board_idx in 0..self.striked.len() {
            if let Some((row, col)) = self.settings.boards[board_idx].position(call) {
                self.striked[board_idx][row][col] = true;
                self.check_winner(board_idx, row, col, call);
            };
        }
    }

    fn check_winner(&mut self, board_idx:usize, row_idx: usize, col_idx:usize, call: usize)
    {
        if !self.winners.iter().find(|&&(board, _)|board == board_idx).is_some() {
            if     (0..BINGO_SIZE).map(|i|self.striked[board_idx][row_idx][i])
                                  .all(|striked|striked)
                || (0..BINGO_SIZE).map(|i|self.striked[board_idx][i][col_idx])
                                  .all(|striked|striked) {
                let mut score = 0;
                for row in 0..BINGO_SIZE {
                    for col in 0..BINGO_SIZE {
                        if !self.striked[board_idx][row][col] {
                            score += self.settings.boards[board_idx].0[row][col];
                        }

                    }
                }
                self.winners.push((board_idx, call * score));
            }
        }
    }

    fn last_winner_score(&self) -> usize {
        self.winners.last().unwrap().1
    }
}

pub fn parse_input(input: &str) -> BingoSettings {
    BingoSettings::from_str(&input)
}

pub fn task1(bingo: &BingoSettings) -> usize
{
    Bingo::new(bingo).task(1)
}

pub fn task2(bingo: &BingoSettings) -> usize
{
    Bingo::new(bingo).task(bingo.boards.len())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_task1() {
        let bingo = BingoSettings::from_str(&DATA);
        assert_eq!(task1(&bingo), 4512);
    }


    #[test]
    fn test_task2() {
        let bingo = BingoSettings::from_str(&DATA);
        assert_eq!(task2(&bingo), 1924);
    }
}