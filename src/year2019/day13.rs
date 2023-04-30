use super::computer::Computer;
use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<isize>> {
    Computer::prepare_code(input)
}

use std::collections::HashMap;

const EMPTY: isize = 0;
const WALL: isize = 1;
const BLOCK: isize = 2;
const _PADDLE: isize = 3;
const _BALL: isize = 4;

pub fn task1(code: &[isize]) -> Result<usize> {
    let mut computer = Computer::new(code);
    computer.run()?;
    let mut grid = HashMap::new();
    while let (Ok(x), Ok(y), Ok(t)) =
        (computer.read(), computer.read(), computer.read())
    {
        grid.insert((x, y), t);
    }
    Ok(grid.values().filter(|&&v| v == BLOCK).count())
}

pub fn task2(code: &[isize]) -> Result<isize> {
    //pre-run to calculate parameters of field
    let mut computer = Computer::new(code);
    computer.run()?;
    let mut grid = HashMap::new();
    while let (Ok(x), Ok(y), Ok(t)) =
        (computer.read(), computer.read(), computer.read())
    {
        grid.insert((x, y), t);
    }
    let width = grid.keys().map(|&p| p.0).max().unwrap() + 1;
    //let height = grid.keys().map(|&p| p.1).max().unwrap();
    let last_row = code
        .windows(width as usize)
        .enumerate()
        .fold(None, |last, (i, wnd)| {
            if wnd[0] == WALL
                && wnd[1..wnd.len() - 1].iter().all(|&x| x == EMPTY)
                && wnd[wnd.len() - 1] == WALL
            {
                Some(i)
            } else {
                last
            }
        })
        .ok_or_else(|| task_error!("Impossible!"))?;
    let mut blocks: HashMap<_, _> =
        grid.iter().filter(|(_, &t)| t == BLOCK).collect();

    let mut code = code.to_vec();
    code[0] = 2;
    for i in 1..width - 1 {
        code[last_row + i as usize] = WALL;
    }
    let mut computer = Computer::new(&code);
    let mut score = 0;
    while !blocks.is_empty() {
        computer.run()?;
        while let (Ok(x), Ok(y), Ok(t)) =
            (computer.read(), computer.read(), computer.read())
        {
            if x >= 0 {
                if t != BLOCK {
                    blocks.remove(&(x, y));
                }
            } else {
                score = t;
            }
        }
        computer.write(0);
    }
    Ok(score)
}
