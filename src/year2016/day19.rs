use crate::*;

use std::collections::VecDeque;

pub fn parse_input(input: &str) -> Result<usize> {
    Ok(input.trim().parse()?)
}

pub fn task1(input: &usize) -> Result<usize> {
    let l = input.ilog2();
    Ok(((input - (1 << l)) << 1) + 1)
}

pub fn task2(&input: &usize) -> Result<usize> {
    let mut left: VecDeque<_> = (1..=(input + 1) / 2).collect();
    let mut right: VecDeque<_> = ((input + 3) / 2..=input).collect();
    while !right.is_empty() {
        if left.len() == right.len() {
            right.pop_front();
        } else {
            left.pop_back();
        }
        if let (Some(&l), Some(&r)) = (left.front(), right.front()) {
            right.push_back(l);
            left.push_back(r);
            left.pop_front();
            right.pop_front();
        }
    }
    left.front()
        .copied()
        .ok_or_else(|| task_error!("Solution not found"))
}
