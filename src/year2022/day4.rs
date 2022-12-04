pub struct Assignment {
    left: usize,
    right: usize,
    range: std::ops::RangeInclusive<usize>,
}

impl Assignment {
    fn new(input: &str) -> Assignment {
        let mut input = input.split('-');
        let left = input.next().unwrap().parse().unwrap();
        let right = input.next().unwrap().parse().unwrap();
        Assignment {
            left,
            right,
            range: left..=right,
        }
    }
    fn contains(&self, other: &Assignment) -> bool {
        self.range.contains(&other.left) && self.range.contains(&other.right)
    }
    fn overlaps(&self, other: &Assignment) -> bool {
        self.range.contains(&other.left)
            || self.range.contains(&other.right)
            || other.contains(self)
    }
}

pub fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|line| {
            let mut pair = line.split(',');
            (
                Assignment::new(pair.next().unwrap()),
                Assignment::new(pair.next().unwrap()),
            )
        })
        .collect()
}

fn filter_count<T, F>(assignments: &[T], f: F) -> usize
where
    F: FnMut(&&T) -> bool,
{
    assignments.iter().filter(f).count()
}

pub fn task1(assignments: &[(Assignment, Assignment)]) -> usize {
    filter_count(assignments, |&(left, right)| {
        left.contains(right) || right.contains(left)
    })
}

pub fn task2(assignments: &[(Assignment, Assignment)]) -> usize {
    filter_count(assignments, |&(left, right)| left.overlaps(right))
}
