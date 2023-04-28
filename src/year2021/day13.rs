use super::super::common::Error::TaskError;
use super::aoc::*;

#[derive(Copy, Clone)]
pub enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Clone)]
pub struct InvisiblePaper {
    dots: std::collections::HashSet<(i32, i32)>,
    folds: Vec<Fold>,
}

impl InvisiblePaper {
    fn do_fold(&mut self, fold: Fold) {
        self.dots = self
            .dots
            .iter()
            .map(|dot| match fold {
                Fold::X(value) => (value - (value - dot.0).abs(), dot.1),
                Fold::Y(value) => (dot.0, value - (value - dot.1).abs()),
            })
            .collect();
    }
}

pub fn parse_input(input: &str) -> Result<InvisiblePaper> {
    let lines = input.lines();
    let mut dots_done = false;
    let mut data = InvisiblePaper {
        dots: std::collections::HashSet::new(),
        folds: Vec::new(),
    };
    for line in lines {
        if line.is_empty() {
            dots_done = true;
        } else if !dots_done {
            data.dots
                .insert(scan_fmt::scan_fmt!(line, "{},{}", i32, i32)?);
        } else {
            let (coord, value) =
                scan_fmt::scan_fmt!(line, "fold along {1[xy]}={}", char, i32)?;
            let fold = match coord {
                'x' => Fold::X(value),
                'y' => Fold::Y(value),
                other => {
                    return Err(TaskError(format!(
                        "Impossible value '{other}'!"
                    )))
                }
            };
            data.folds.push(fold);
        }
    }
    Ok(data)
}

pub fn task1(data: &InvisiblePaper) -> Result<usize> {
    let mut data = data.clone();
    data.do_fold(data.folds[0]);
    Ok(data.dots.len())
}

pub fn task2(data: &InvisiblePaper) -> Result<String> {
    let folds = &data.folds;
    let mut data = data.clone();
    for &fold in folds {
        data.do_fold(fold);
    }
    let max_x = data
        .dots
        .iter()
        .map(|(x, _)| x)
        .max()
        .ok_or_else(|| TaskError("No dots!".to_string()))?
        + 1;
    let max_y = data
        .dots
        .iter()
        .map(|(_, y)| y)
        .max()
        .ok_or_else(|| TaskError("No dots!".to_string()))?
        + 1;
    let mut result = String::new();
    for y in 0..max_y {
        for x in 0..max_x {
            result.push(if data.dots.contains(&(x, y)) {
                '#'
            } else {
                '.'
            });
        }
        result.push('\n');
    }
    Ok(result)
}
