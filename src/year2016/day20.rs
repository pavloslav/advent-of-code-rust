use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<(usize, usize)>> {
    let mut intervals: Vec<(usize, usize)> = Vec::new();
    for scan in input
        .lines()
        .map(|line| scan_fmt::scan_fmt!(line, "{}-{}", usize, usize))
    {
        let (left, right) = scan?;
        if right < left {
            unreachable!();
        }
        let (left, right) =
            (left.min(u32::MAX as usize), right.min(u32::MAX as usize));
        intervals.push((left, right));
    }
    intervals.sort_by_key(|&(left, _)| left);
    let mut i = 0;
    while i + 1 < intervals.len() {
        if let Some(int) = join(intervals[i], intervals[i + 1]) {
            intervals[i] = int;
            intervals.remove(i + 1);
        } else {
            i += 1;
        }
    }
    Ok(intervals)
}

fn join(int1: (usize, usize), int2: (usize, usize)) -> Option<(usize, usize)> {
    let range1 = int1.0..=int1.1;
    let range2 = int2.0..=int2.1;
    if range1.contains(&int2.0)
        || range1.contains(&int2.1)
        || range2.contains(&int1.0)
        || range2.contains(&int1.1)
        || int1.0 == int2.1 + 1
        || int2.0 == int1.1 + 1
    {
        Some((int1.0.min(int2.0), int1.1.max(int2.1)))
    } else {
        None
    }
}

pub fn task1(intervals: &[(usize, usize)]) -> Result<usize> {
    Ok(intervals[0].1 + 1)
}

pub fn task2(intervals: &[(usize, usize)]) -> Result<usize> {
    let mut allowed = u32::MAX as usize;
    for &(left, right) in intervals.iter().rev() {
        allowed -= right + 1 - left;
    }
    Ok(allowed + 1)
}
