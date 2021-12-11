fn task<F>(crabs: &[i32], fuel: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let total = |pos| crabs.iter().map(|&crab| fuel(crab, pos)).sum();
    let mut pos = (crabs.last().unwrap() - crabs.first().unwrap()) / 2;
    let mut step = pos / 2;
    loop {
        let dist = total(pos);
        let dist_l = total(pos - 1);
        let dist_r = total(pos + 1);
        if dist <= dist_l && dist <= dist_r {
            return dist;
        } else if dist_r < dist {
            pos += step;
        } else {
            pos -= step;
        }
        step = std::cmp::max(1, step / 2);
    }
}

pub fn parse_input(input: &str) -> Vec<i32> {
    input.split(',')
         .map(|x| x.parse().unwrap())
         .collect()
}

pub fn task1(crabs: &[i32]) -> i32 {
    task(crabs, |crab, pos| (pos - crab).abs())
}

pub fn task2(crabs: &[i32]) -> i32 {
    task(crabs, |crab, pos| {
        (pos - crab).abs() * ((pos - crab).abs() + 1) / 2
    })
}