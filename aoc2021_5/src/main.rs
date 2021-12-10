type Line = ((i32, i32),(i32, i32));

const FIELD_SIZE: usize = 1000;

fn parse_vents(lines: &str) -> Vec<Line> {
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
    let mut field: Vec<Vec<i32>> = std::iter::repeat(std::iter::repeat(0).take(FIELD_SIZE).collect()).take(FIELD_SIZE).collect();
    for vent in vents {
        if is_diagonals_needed || !is_diagonal(vent) {
            let (mut x, mut y) = vent.0;
            let (dx, dy) = (-ordering_to_i32(x.cmp(&vent.1.0)),-ordering_to_i32(y.cmp(&vent.1.1)));
            loop {
                field[x as usize][y as usize] += 1;
                if (x,y) == vent.1 {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
    }
    field.iter().map(|line|line.iter().filter(|&&x|x>1).count() as i32).sum()
}

fn task1(vents: &[Line]) -> i32
{
    task(vents, false)
}


fn task2(vents: &[Line]) -> i32
{
    task(vents, true)
}

fn main() {
    use std::time::Instant;
    let input = aoc::get_input_from_ini_with_year("5","2021").unwrap();
    let vents = parse_vents(&input);
    let start = Instant::now();
    let r1 = task1(&vents);
    let r2 = task2(&vents);
    println!("mcs={}", start.elapsed().as_micros());
    println!("Result1: {}", r1);
    println!("Result2: {}", r2);
}