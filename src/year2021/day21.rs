#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Player {
        Player { position, score: 0 }
    }
}

#[derive(Clone)]
pub struct Game {
    players: Vec<Player>,
    turn: usize,
    dice_value: usize,
    dice_rolled: usize,
}

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.split(": ").skip(1).next().unwrap().parse().unwrap())
        .collect()
}

impl Game {
    const WIN_SCORE: usize = 1000;
    const BOARD_SIZE: usize = 10;
    const DICE_SIZE: usize = 100;

    fn new(player_pos: &Vec<usize>) -> Game {
        Game {
            players: player_pos.iter().map(|&pos| Player::new(pos)).collect(),
            turn: 0,
            dice_value: 1,
            dice_rolled: 0,
        }
    }
    fn step(&mut self) {
        let whos_turn = self.turn % self.players.len();
        self.players[whos_turn].position = (self.players[whos_turn].position
            + self.roll()
            + self.roll()
            + self.roll()
            - 1)
            % Game::BOARD_SIZE
            + 1;
        self.players[whos_turn].score += self.players[whos_turn].position;
        self.turn += 1;
    }
    fn roll(&mut self) -> usize {
        let roll = self.dice_value;
        self.dice_rolled += 1;
        self.dice_value = self.dice_value % Game::DICE_SIZE + 1;
        roll
    }
    fn ended(&self) -> bool {
        self.players
            .iter()
            .any(|player| player.score >= Game::WIN_SCORE)
    }
    fn less_score(&self) -> usize {
        self.players
            .iter()
            .map(|player| player.score)
            .min()
            .unwrap()
    }
}

pub fn task1(input: &Vec<usize>) -> usize {
    let mut game = Game::new(input);
    while !game.ended() {
        game.step();
    }
    game.less_score() * game.dice_rolled
}

#[derive(Clone, Debug)]
pub struct DiracGame {
    //universes: Vec<(Vec<Player>, u128)>,
    universes: std::collections::HashMap<Vec<Player>, u128>,
    wins: Vec<u128>,
    turn: usize,
}

impl DiracGame {
    //roll, number of universes
    const DIRAC_CUBE: [(usize, u128); 7] =
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    const WIN_SCORE: usize = 21;
    const BOARD_SIZE: usize = 10;

    fn new(pos: &Vec<usize>) -> DiracGame {
        DiracGame {
            universes: [(
                pos.iter().map(|&position| Player::new(position)).collect(),
                1,
            )]
            .into(),

            wins: vec![0; pos.len()],
            turn: 0,
        }
    }
    fn ended(&self) -> bool {
        self.universes.is_empty()
    }

    fn step(&mut self) {
        let whos_turn = self.turn % self.wins.len();
        let mut universes = std::collections::HashMap::new();
        for (players, old_universes) in &self.universes {
            for (roll, roll_universes) in DiracGame::DIRAC_CUBE {
                let new_position = (players[whos_turn].position + roll - 1)
                    % DiracGame::BOARD_SIZE
                    + 1;
                let new_score = players[whos_turn].score + new_position;
                if new_score >= DiracGame::WIN_SCORE {
                    self.wins[whos_turn] += old_universes * roll_universes;
                } else {
                    let mut new_players = players.clone();
                    new_players[whos_turn].position = new_position;
                    new_players[whos_turn].score = new_score;
                    *universes.entry(new_players).or_insert(0) +=
                        old_universes * roll_universes;
                }
            }
        }
        self.universes = universes;
        self.turn += 1;
    }
}

pub fn task2(input: &Vec<usize>) -> u128 {
    let mut game = DiracGame::new(input);
    while !game.ended() {
        game.step();
    }
    *game.wins.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let data = parse_input(
            &"Player 1 starting position: 4
Player 2 starting position: 8",
        );
        assert_eq!(task1(&data), 739785);
    }

    #[test]
    fn test_task2() {
        let data = parse_input(
            &"Player 1 starting position: 4
Player 2 starting position: 8",
        );
        assert_eq!(task2(&data), 444356092776315);
    }
}
