use crate::*;

#[derive(Clone)]
pub struct Node {
    x: usize,
    y: usize,
    used: i32,
    avail: i32,
}

pub fn parse_input(input: &str) -> AocResult<Vec<Node>> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let (x, y, size, used, avail, _): (usize, usize, i32, i32, i32, i32) =
                prse::try_parse!(line, "/dev/grid/node-x{}-y{} {}T {}T {}T {}%")?;
            if used + avail != size {
                Err(aoc_error!(
                    "Used={used}, avail={avail}, together {}, but size is {size}!",
                    used + avail
                ))?
            }
            Ok(Node { x, y, used, avail })
        })
        .collect()
}

pub fn task1(nodes: &[Node]) -> AocResult<usize> {
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

pub fn task2(nodes: &[Node]) -> AocResult<String> {
    let width = nodes
        .iter()
        .map(|node| node.x)
        .max()
        .ok_or_else(|| aoc_error!("No nodes!!!1111"))?
        + 1;
    let height = nodes
        .iter()
        .map(|node| node.y)
        .max()
        .ok_or_else(|| aoc_error!("No nodes!!!1111"))?
        + 1;
    let mut map = vec![vec![b'X'; width]; height];
    for node in nodes {
        map[node.y][node.x] = match node.used {
            0 => b'_',
            x if x < 100 => b'.',
            _ => b'#',
        };
    }
    if map[0][0] != b'.' || map[0][width - 1] != b'.' {
        return Err(aoc_error!(
            "Node 0,0 is {}, node 0,{} is {}!",
            map[0][0],
            width - 1,
            map[0][width - 1]
        ));
    }
    map[0][0] = b'E';
    map[0][width - 1] = b'<';

    Ok(map
        .iter()
        .map(|row| std::str::from_utf8(row).map_err(|_| aoc_error!("Impossible, it's ASCII!")))
        .collect::<AocResult<Vec<_>>>()?
        .join("\n"))
}
