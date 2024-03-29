#[derive(Clone, prse::Parse)]
#[prse = "position=<{x}, {y}> velocity=<{vx}, {vy}>"]
pub struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
    fn step_back(&mut self) {
        self.x -= self.vx;
        self.y -= self.vy;
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<(usize, Vec<Point>)> {
    let pts: Vec<Point> = input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{}")?))
        .collect::<anyhow::Result<_>>()?;
    find_narrowest(&pts)
}

fn find_narrowest(input: &[Point]) -> anyhow::Result<(usize, Vec<Point>)> {
    let mut points = input.to_vec();
    let mut min_dist = i32::MAX;
    for step in 0.. {
        for pt in &mut points {
            pt.step();
        }
        let min_x = points
            .iter()
            .map(|pt| pt.x)
            .min()
            .ok_or(anyhow::anyhow!("No points!"))?;
        let max_x = points
            .iter()
            .map(|pt| pt.x)
            .max()
            .ok_or(anyhow::anyhow!("No points!"))?;
        if min_dist > max_x - min_x {
            min_dist = max_x - min_x;
        } else {
            for pt in &mut points {
                pt.step_back();
            }
            return Ok((step, points));
        }
    }
    Err(anyhow::anyhow!("unreachable"))
}

pub fn task1((_, points): &(usize, Vec<Point>)) -> anyhow::Result<String> {
    let min_x = points
        .iter()
        .map(|pt| pt.x)
        .min()
        .ok_or(anyhow::anyhow!("No points!"))?;
    let max_x = points
        .iter()
        .map(|pt| pt.x)
        .max()
        .ok_or(anyhow::anyhow!("No points!"))?;
    let min_y = points
        .iter()
        .map(|pt| pt.y)
        .min()
        .ok_or(anyhow::anyhow!("No points!"))?;
    let max_y = points
        .iter()
        .map(|pt| pt.y)
        .max()
        .ok_or(anyhow::anyhow!("No points!"))?;
    let width = (max_x - min_x + 2) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut result = vec![b'.'; width * height];
    for line in 1..=height {
        result[line * width - 1] = b'\n';
    }
    for pt in points {
        result[(pt.y - min_y) as usize * width + (pt.x - min_x) as usize] = b'#';
    }
    Ok(String::from_utf8(result)?)
}

pub fn task2((steps, _): &(usize, Vec<Point>)) -> anyhow::Result<usize> {
    Ok(*steps)
}
