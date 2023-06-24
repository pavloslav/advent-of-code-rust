use crate::*;

pub fn parse_input(input: &str) -> Result<(usize, usize)> {
    Ok(scan_fmt::scan_fmt!(
        input,
        "{} players; last marble is worth {} points",
        usize,
        usize
    )?)
}

fn marble_game(players: usize, marbles: usize) -> Result<usize> {
    use std::collections::VecDeque;
    let mut circle = VecDeque::from([0]);
    let mut players = vec![0; players];
    for marble in 1..=marbles {
        if marble % 23 != 0 {
            circle.rotate_left(1);
            circle.push_back(marble)
        } else {
            let player_idx = marble % players.len();
            players[player_idx] += marble;
            circle.rotate_right(7);
            players[player_idx] +=
                circle.pop_back().ok_or(aoc_error!("Empry circle!"))?;
            circle.rotate_left(1);
        }
    }

    players
        .iter()
        .max()
        .copied()
        .ok_or(aoc_error!("No players!"))
}

pub fn task1(&(players, marbles): &(usize, usize)) -> Result<usize> {
    marble_game(players, marbles)
}

pub fn task2(&(players, marbles): &(usize, usize)) -> Result<usize> {
    marble_game(players, 100 * marbles)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&(9, 25)).unwrap(), 32);
        assert_eq!(task1(&(10, 1618)).unwrap(), 8317);
        assert_eq!(task1(&(13, 7999)).unwrap(), 146373);
        assert_eq!(task1(&(17, 1104)).unwrap(), 2764);
        assert_eq!(task1(&(21, 6111)).unwrap(), 54718);
        assert_eq!(task1(&(30, 5807)).unwrap(), 37305);
    }
}
