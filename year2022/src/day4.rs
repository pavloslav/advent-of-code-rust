pub struct Assignment {
    left: usize,
    right: usize,
    range: std::ops::RangeInclusive<usize>,
}

impl Assignment {
    fn new(left: usize, right: usize) -> Assignment {
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

pub fn parse_input(input: &str) -> anyhow::Result<Vec<(Assignment, Assignment)>> {
    input
        .lines()
        .map(|line| {
            let (l1, r1, l2, r2) = prse::try_parse!(line, "{}-{},{}-{}")?;
            Ok((Assignment::new(l1, r1), Assignment::new(l2, r2)))
        })
        .collect()
}

fn filter_count<T, F>(assignments: &[T], f: F) -> usize
where
    F: FnMut(&&T) -> bool,
{
    assignments.iter().filter(f).count()
}

pub fn task1(assignments: &[(Assignment, Assignment)]) -> anyhow::Result<usize> {
    Ok(filter_count(assignments, |&(left, right)| {
        left.contains(right) || right.contains(left)
    }))
}

pub fn task2(assignments: &[(Assignment, Assignment)]) -> anyhow::Result<usize> {
    Ok(filter_count(assignments, |&(left, right)| {
        left.overlaps(right)
    }))
}
