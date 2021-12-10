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

fn task1(crabs: &[i32]) -> i32 {
    task(crabs, |crab, pos| (pos - crab).abs())
}

fn task2(crabs: &[i32]) -> i32 {
    task(crabs, |crab, pos| {
        (pos - crab).abs() * ((pos - crab).abs() + 1) / 2
    })
}

fn main() {
    let input = aoc::get_input_from_ini_with_year("7", "2021").unwrap();
    let crabs: Vec<_> = input.split(',').map(|x| x.parse().unwrap()).collect();
    println!("Result1: {}", task1(&crabs));
    println!("Result2: {}", task2(&crabs));
}
