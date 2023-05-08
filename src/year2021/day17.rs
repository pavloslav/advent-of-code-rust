use crate::*;

type Rect = ((i32, i32), (i32, i32));

pub fn parse_input(input: &str) -> Result<Rect> {
    let (x1, x2, y1, y2) = scan_fmt::scan_fmt!(
        input,
        "target area: x={}..{}, y={}..{}",
        i32,
        i32,
        i32,
        i32
    )?;
    Ok(((x1, y1), (x2, y2)))
}

pub fn task1(input: &Rect) -> Result<i32> {
    let &((x1, y1), (x2, _y2)) = input;
    let vx = (((1 + 8 * x2) as f64).sqrt() as i32 - 1) / 2;
    if vx * (vx + 1) / 2 < x1 {
        return Err(task_error!("Probe don't stop, vx={vx}"));
    }
    let vy = (y1 + 1).abs();
    Ok(vy * (vy + 1) / 2)
}

fn get_all_velocities(input: &Rect) -> std::collections::HashSet<(i32, i32)> {
    let &((x1, y1), (x2, y2)) = input;
    let max_t = 2 * y1.abs();

    let mut result = std::collections::HashSet::new();

    for t in 1..=max_t {
        let mut vx_min = (x1 + t * (t - 1) / 2 + t - 1) / t;
        if vx_min < t {
            vx_min = ((((1 + 8 * x1) as f64).sqrt() - 1.0) / 2.0).ceil() as i32;
        }
        let mut vx_max = (x2 + t * (t - 1) / 2) / t;
        if vx_max < t {
            vx_max = ((((1 + 8 * x2) as f64).sqrt() - 1.0) / 2.0) as i32;
        }
        let vy_min = ((y1 + t * (t - 1) / 2) as f64 / t as f64).ceil() as i32;
        let vy_max = ((y2 + t * (t - 1) / 2) as f64 / t as f64).floor() as i32;

        for vx in vx_min..=vx_max {
            for vy in vy_min..=vy_max {
                result.insert((vx, vy));
            }
        }
    }
    result
}

pub fn task2(input: &Rect) -> Result<usize> {
    Ok(get_all_velocities(input).len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&((20, -10), (30, -5))).unwrap(), 45);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&((20, -10), (30, -5))).unwrap(), 112);
    }

    use itertools::Itertools;

    #[test]
    fn test_get_all_velocities() {
        let expected_str = "23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
8,-2    27,-8   30,-5   24,-7";
        let expected = expected_str
            .split_whitespace()
            .map(|pair| {
                pair.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        let result = get_all_velocities(&((20, -10), (30, -5)));
        if result != expected {
            let extra: Vec<_> = result.difference(&expected).collect();
            if !extra.is_empty() {
                println!("Extra values: {:?}", extra);
            }
            let lost: Vec<_> = expected.difference(&result).collect();
            if !lost.is_empty() {
                println!("Lost values: {:?}", lost);
            }
            panic!();
        }
    }
}
