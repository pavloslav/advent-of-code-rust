pub type Line = ((i32, i32),(i32, i32));

pub fn parse_input(lines: &str) -> Vec<Line> {
    lines.lines()
         .map(|line|{
            let mut line = line.split(" -> ");
            let mut pt1 = line.next().unwrap().split(',');
            let mut pt2 = line.next().unwrap().split(',');
            ((pt1.next().unwrap().parse().unwrap(), pt1.next().unwrap().parse().unwrap()),
             (pt2.next().unwrap().parse().unwrap(), pt2.next().unwrap().parse().unwrap()))
        })
        .collect()
}

fn is_diagonal(line: &Line) -> bool {
    line.0.0 != line.1.0 && line.0.1 != line.1.1
}

fn ordering_to_i32(order: std::cmp::Ordering) -> i32{
    match order {
        std::cmp::Ordering::Greater =>  1,
        std::cmp::Ordering::Equal   =>  0,
        std::cmp::Ordering::Less    => -1,
    }
}

fn task(vents: &[Line], is_diagonals_needed: bool) -> i32
{
    use std::collections::HashMap;
    let mut known_vents : HashMap<(i32, i32), i32>= HashMap::new();
    for vent in vents {
        if is_diagonals_needed || !is_diagonal(vent) {
            let (mut x, mut y) = vent.0;
            let (dx, dy) = (-ordering_to_i32(x.cmp(&vent.1.0)),-ordering_to_i32(y.cmp(&vent.1.1)));
            loop {
                *known_vents.entry((x,y)).or_insert(0) += 1;
                if (x,y) == vent.1 {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
    }
    known_vents.values().filter(|&&x|x>1).count() as i32
}

pub fn task1(vents: &[Line]) -> i32
{
    task(vents, false)
}


pub fn task2(vents: &[Line]) -> i32
{
    task(vents, true)
}