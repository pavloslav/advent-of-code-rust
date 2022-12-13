use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    Computer::prepare_code(input)
}

use std::collections::HashMap;

pub fn task1(code: &[isize]) -> usize {
    let mut computer = Computer::new(code);
    computer.run();
    let mut grid = HashMap::new();
    while let (Some(x), Some(y), Some(t)) =
        (computer.read(), computer.read(), computer.read())
    {
        grid.insert((x, y), t);
    }
    grid.values().filter(|&&v| v == 2).count()
}

fn image(tile: isize) -> char {
    [' ', '#', 'x', '-', 'o'][tile as usize]
}

pub fn task2(code: &[isize]) -> isize {
    let mut computer = Computer::new(code);
    *computer.memory.get_mut(&0).unwrap() = 2;
    let mut score = 0;
    //let mut grid = vec![vec![0; 44]; 20];
    let window = pancurses::initscr();
    pancurses::noecho();
    //window.nodelay(true);
    loop {
        computer.run();
        while let (Some(x), Some(y), Some(t)) =
            (computer.read(), computer.read(), computer.read())
        {
            if x >= 0 {
                window.mvprintw(y as i32, x as i32, String::from(image(t)));
                //grid[y as usize][x as usize] = t;
            } else {
                score = t;
            }
        }
        /*
        println!("score: {}", score);
        for line in &grid {
            for &tile in line {
                print!("{}", image(tile))
            }
            println!();
        }*/
        window.mvprintw(20, 0, format!("score: {}", score));
        window.refresh();
        std::thread::sleep(std::time::Duration::from_millis(500));
        match window.getch() {
            Some(pancurses::Input::Character('a')) => computer.write(-1),
            Some(pancurses::Input::Character('d')) => computer.write(1),
            _ => computer.write(0),
        }
    }
    score
}
