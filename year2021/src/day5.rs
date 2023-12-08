#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, prse::Parse)]
#[prse = "{x},{y}"]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(prse::Parse)]
#[prse = "{pt1} -> {pt2}"]
pub struct Line {
    pt1: Point,
    pt2: Point,
}

const FIELD_SIZE: usize = 1000;

pub fn parse_input(lines: &str) -> anyhow::Result<Vec<Line>> {
    lines
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{}")?))
        .collect()
}

fn is_diagonal(line: &Line) -> bool {
    line.pt1.x != line.pt2.x && line.pt1.y != line.pt2.y
}

fn task(vents: &[Line], is_diagonals_needed: bool) -> anyhow::Result<i32> {
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
                    anyhow::bail!("Point {pt:?} is negative");
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

pub fn task1(vents: &[Line]) -> anyhow::Result<i32> {
    task(vents, false)
}

pub fn task2(vents: &[Line]) -> anyhow::Result<i32> {
    task(vents, true)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 5);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input(INPUT).unwrap()).unwrap(), 12);
    }
}
