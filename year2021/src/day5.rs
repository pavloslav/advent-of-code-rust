use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Line {
    pt1: Point,
    pt2: Point,
}

const FIELD_SIZE: usize = 1000;

pub fn parse_input(lines: &str) -> AocResult<Vec<Line>> {
    lines
        .lines()
        .map(|line| {
            let (x1, y1, x2, y2) = prse::try_parse!(line, "{},{} -> {},{}", i32, i32, i32, i32)?;
            Ok(Line {
                pt1: Point { x: x1, y: y1 },
                pt2: Point { x: x2, y: y2 },
            })
        })
        .collect()
}

fn is_diagonal(line: &Line) -> bool {
    line.pt1.x != line.pt2.x && line.pt1.y != line.pt2.y
}

fn task(vents: &[Line], is_diagonals_needed: bool) -> AocResult<i32> {
    let mut field: Vec<Vec<i32>> =
        std::iter::repeat(std::iter::repeat(0).take(FIELD_SIZE).collect())
            .take(FIELD_SIZE)
            .collect();
    for vent in vents {
        if is_diagonals_needed || !is_diagonal(vent) {
            let mut pt = vent.pt1;
            let (dx, dy) = ((vent.pt2.x - pt.x).signum(), (vent.pt2.y - pt.y).signum());
            loop {
                if pt.x < 0 || pt.y < 0 {
                    return Err(aoc_error!("Point {pt:?} is negative"));
                }
                field[pt.x as usize][pt.y as usize] += 1;
                if pt == vent.pt2 {
                    break;
                }
                pt.x += dx;
                pt.y += dy;
            }
        }
    }
    let sum = field
        .iter()
        .map(|line| line.iter().filter(|&&x| x > 1).count() as i32)
        .sum();
    Ok(sum)
}

pub fn task1(vents: &[Line]) -> AocResult<i32> {
    task(vents, false)
}

pub fn task2(vents: &[Line]) -> AocResult<i32> {
    task(vents, true)
}
