use crate::*;

use std::collections::HashSet;

pub struct Rect {
    number: i32,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

pub fn parse_input(input: &str) -> Result<Vec<Rect>> {
    input
        .lines()
        .map(|line| {
            let (number, left, top, width, height) = scan_fmt::scan_fmt!(
                line,
                "#{} @ {},{}: {}x{}",
                i32,
                i32,
                i32,
                i32,
                i32
            )?;
            Ok(Rect {
                number,
                left,
                top,
                right: left + width,
                bottom: top + height,
            })
        })
        .collect()
}

pub fn task1(input: &[Rect]) -> Result<usize> {
    let mut multiclaimed = HashSet::new();
    for (i, r1) in input.iter().enumerate() {
        for r2 in &input[i + 1..] {
            for x in r1.left.max(r2.left)..r1.right.min(r2.right) {
                for y in r1.top.max(r2.top)..r1.bottom.min(r2.bottom) {
                    multiclaimed.insert((x, y));
                }
            }
        }
    }
    Ok(multiclaimed.len())
}

pub fn task2(input: &[Rect]) -> Result<i32> {
    let mut not_overlapped: HashSet<_> =
        input.iter().map(|r| r.number).collect();
    for (i, r1) in input.iter().enumerate() {
        for r2 in &input[i + 1..] {
            if r1.right.min(r2.right) >= r1.left.max(r2.left)
                && r1.bottom.min(r2.bottom) >= r1.top.max(r2.top)
            {
                not_overlapped.remove(&r1.number);
                not_overlapped.remove(&r2.number);
            }
        }
    }

    if not_overlapped.len() != 1 {
        Err(aoc_error!("{} results found", not_overlapped.len()))
    } else {
        Ok(*not_overlapped.iter().next().unwrap())
    }
}
