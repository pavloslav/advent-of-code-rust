#[derive(Copy, Clone)]
pub enum Fold {
    X,
    Y,
}

#[derive(Clone)]
pub struct InvisiblePaper {
    dots: std::collections::HashSet<(i32, i32)>,
    folds: Vec<(Fold, i32)>,
}

impl InvisiblePaper {
    fn do_fold(&mut self, fold: (Fold, i32)) {
        self.dots = self.dots
                        .iter()
                        .map(|dot|
                            match fold.0 {
                                Fold::X => (fold.1 - (fold.1-dot.0).abs(), dot.1),
                                Fold::Y => (dot.0, fold.1 - (fold.1-dot.1).abs()),
                            })
                        .collect();
    }
}

pub fn parse_input(input: &str) -> InvisiblePaper {
    let lines = input.lines();
    let mut dots_done = false;
    let mut data = InvisiblePaper {
        dots: std::collections::HashSet::new(), 
        folds: Vec::new(),
    };
    for line in lines {
        if line.is_empty() {
            dots_done = true;
        } else {
            if !dots_done {
                let mut dot = line.split(',');
                data.dots.insert((dot.next().unwrap().parse().unwrap(),
                                dot.next().unwrap().parse().unwrap()));
            } else {
                let mut fold = line.split('=');
                let fold_type = if fold.next().unwrap().chars().nth(11)==Some('x') {
                    Fold::X
                } else {
                    Fold::Y
                };
                data.folds.push((fold_type,fold.next().unwrap().parse().unwrap()));
            }
        }
    }
    data
}

pub fn task1(data: &InvisiblePaper) -> usize {
    let mut data = data.clone();
    data.do_fold(data.folds[0]);
    data.dots.len()
}

pub fn task2(data: &InvisiblePaper) -> usize {
    let folds = &data.folds;
    let mut data = data.clone();
    for &fold in folds {
        data.do_fold(fold);
    }
    let max_x = data.dots.iter().map(|(x, _)|x).max().unwrap() + 1;
    let max_y = data.dots.iter().map(|(_, y)|y).max().unwrap() + 1;
    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", if data.dots.contains(&(x,y)) {"#"} else {"."});
        }
        println!("");
    }
    0
}
