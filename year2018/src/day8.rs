pub struct Tree {
    children: Vec<Tree>,
    metadata: Vec<i32>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            children: vec![],
            metadata: vec![],
        }
    }
    fn metadata_sum(&self) -> i32 {
        self.metadata.iter().sum::<i32>()
            + self
                .children
                .iter()
                .map(|child| child.metadata_sum())
                .sum::<i32>()
    }
    fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|child_idx| {
                    (child_idx - 1)
                        .try_into()
                        .ok()
                        .and_then(|idx: usize| self.children.get(idx))
                        .map(|tree| tree.value())
                })
                .sum()
        }
    }
}

fn parse_tree(
    input: &mut impl std::iter::Iterator<Item = anyhow::Result<i32>>,
) -> anyhow::Result<Tree> {
    let mut tree = Tree::new();
    let child_count = input.next().ok_or(anyhow::anyhow!("Empty node!"))??;
    let metadata_size = input.next().ok_or(anyhow::anyhow!("No metadata size!"))??;
    for _ in 0..child_count {
        tree.children.push(parse_tree(input)?);
    }
    for _ in 0..metadata_size {
        tree.metadata.push(
            input
                .next()
                .ok_or(anyhow::anyhow!("Not enough metadata!"))??,
        );
    }
    Ok(tree)
}

pub fn parse_input(input: &str) -> anyhow::Result<Tree> {
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse().map_err(|_| anyhow::anyhow!("Not a number!")));
    parse_tree(&mut iter)
}

pub fn task1(input: &Tree) -> anyhow::Result<i32> {
    Ok(input.metadata_sum())
}

pub fn task2(input: &Tree) -> anyhow::Result<i32> {
    Ok(input.value())
}
