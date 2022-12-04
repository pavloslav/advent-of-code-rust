pub struct Step {
    direction: char,
    length: i32,
}

impl Step {
    fn new(input: &str) -> Step {
        Step {
            direction: input.chars().next().unwrap(),
            length: input[1..].parse().unwrap(),
        }
    }
    fn shift(&self) -> (i32, i32) {
        match self.direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("Wrong direction: {}", self.direction),
        }
    }
}

pub fn parse_input(input: &str) -> (Vec<Step>, Vec<Step>) {
    let mut input = input.lines();
    (
        input.next().unwrap().split(',').map(Step::new).collect(),
        input.next().unwrap().split(',').map(Step::new).collect(),
    )
}

use std::collections::HashSet;

fn get_set(steps: &[Step]) -> HashSet<(i32, i32)> {
    let (mut x, mut y) = (0, 0);
    steps
        .iter()
        .flat_map(|step| {
            let (old_x, old_y) = (x, y);
            let shift = step.shift();
            x += step.length * shift.0;
            y += step.length * shift.1;
            (1..=step.length)
                .map(move |i| (old_x + i * shift.0, old_y + i * shift.1))
        })
        .collect()
}

pub fn task1(input: &(Vec<Step>, Vec<Step>)) -> usize {
    let way1 = get_set(&input.0);
    let way2 = get_set(&input.1);
    way1.intersection(&way2)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap() as usize
}

use std::collections::HashMap;

fn get_map(steps: &[Step]) -> HashMap<(i32, i32), usize> {
    let (mut x, mut y, mut index) = (0, 0, 0);
    steps
        .iter()
        .flat_map(|step| {
            let (old_x, old_y, old_index) = (x, y, index);
            let shift = step.shift();
            x += step.length * shift.0;
            y += step.length * shift.1;
            index += step.length as usize;
            (1..=step.length).map(move |i| {
                (
                    (old_x + i * shift.0, old_y + i * shift.1),
                    old_index + i as usize,
                )
            })
        })
        .collect()
}

pub fn task2(input: &(Vec<Step>, Vec<Step>)) -> usize {
    let way1 = get_map(&input.0);
    let way2 = get_map(&input.1);
    way1.iter()
        .filter_map(|(key, length)| {
            if way2.contains_key(key) {
                Some(length + way2[key])
            } else {
                None
            }
        })
        .min()
        .unwrap()
}
