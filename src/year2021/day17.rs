type Rect = ((i32, i32), (i32, i32));

pub fn parse_input(input: &str) -> Rect {
    let mut parts = input["target area: ".len()..].split(", ");
    let mut xs = parts.next().unwrap()["x=".len()..]
        .split("..")
        .map(|x| x.parse().unwrap());
    let mut ys = parts.next().unwrap()["x=".len()..]
        .split("..")
        .map(|x| x.parse().unwrap());
    (
        (xs.next().unwrap(), ys.next().unwrap()),
        (xs.next().unwrap(), ys.next().unwrap()),
    )
}

pub fn task1(input: &Rect) -> i32 {
    let &((x1, y1), (x2, _y2)) = input;
    //x1<=vx*(vx+1)/2<=x2
    //2*x1<=vx**2 + vx<=2*x2
    let vx = (((1 + 8 * x2) as f64).sqrt() as i32 - 1) / 2;
    assert!(vx * (vx + 1) / 2 >= x1, "Probe don't stop, vx={}", vx);
    let vy = (y1 + 1).abs();
    vy * (vy + 1) / 2
}

pub fn task2(_input: &Rect) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&((20, -10), (30, -5))), 45);
    }
}
