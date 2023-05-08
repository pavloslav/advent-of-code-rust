use crate::*;

#[derive(Clone)]
pub struct Node {
    _x: i32,
    _y: i32,
    _size: i32,
    used: i32,
    avail: i32,
}

pub fn parse_input(input: &str) -> Result<Vec<Node>> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let (x, y, size, used, avail) = scan_fmt::scan_fmt!(
                line,
                "/dev/grid/node-x{d}-y{d} {d}T {d}T {d}T {*d}%",
                i32,
                i32,
                i32,
                i32,
                i32
            )?;
            if used+avail != size {
                Err(task_error!("Used={used}, avail={avail}, together {}, but size is {size}!", used+avail))?
            }
            Ok(Node { _x:x, _y:y, _size:size, used, avail })
        })
        .collect()
}

pub fn task1(nodes: &[Node]) -> Result<usize> {
    let mut nodes: Vec<_> = nodes.to_vec();
    nodes.sort_by_key(|node| node.avail);
    let avails: Vec<_> = nodes.iter().map(|node| node.avail).collect();
    let mut pairs = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node.used > 0 {
            let j = avails.binary_search(&node.used).unwrap_or_else(|j| j);
            pairs += avails.len() - j;
            if i >= j {
                pairs -= 1;
            };
        }
    }
    Ok(pairs)
}

pub fn task2(_nodes: &[Node]) -> Result<usize> {
    Err(task_error!("Solution not found"))
}
