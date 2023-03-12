type Connections = std::collections::HashMap<usize, Vec<usize>>;

pub fn parse_input(input: &str) -> Connections {
    static INPUT_REGEX: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| {
            regex::Regex::new(r"(?P<index>\d+) <-> (?P<connected>.*)$").unwrap()
        });
    input
        .lines()
        .map(|line| {
            if let Some(captures) = INPUT_REGEX.captures(line) {
                (
                    captures.name("index").unwrap().as_str().parse().unwrap(),
                    captures
                        .name("connected")
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|s| s.parse().unwrap())
                        .collect(),
                )
            } else {
                panic!()
            }
        })
        .collect()
}

fn floodfill(
    connections: &Connections,
    start: usize,
    value: i32,
    field: &mut [i32],
) {
    let mut to_fill = connections[&start].clone();
    while let Some(tgt) = to_fill.pop() {
        if field[tgt] != value {
            field[tgt] = value;
            to_fill.extend(&connections[&tgt]);
        }
    }
}

pub fn task1(input: &Connections) -> usize {
    let mut zero_connected = vec![-1; input.len()];
    floodfill(input, 0, 0, &mut zero_connected);
    zero_connected.iter().filter(|&&x| x == 0).count()
}

pub fn task2(input: &Connections) -> i32 {
    let mut connect: Vec<i32> = vec![-1; input.len()];
    let mut group_index = 0;
    while let Some((first, _x)) =
        connect.iter().enumerate().find(|&(_, &x)| x == -1)
    {
        floodfill(input, first, group_index, &mut connect);
        group_index += 1;
    }
    group_index
}
