use crate::*;

type Connections = std::collections::HashMap<usize, Vec<usize>>;

pub fn parse_input(input: &str) -> AocResult<Connections> {
    input
        .lines()
        .map(|line| {
            let (program, connections) = prse::try_parse!(line, "{} <-> {:, :0}")?;
            Ok((
                program,
                connections
                    .map(|s: Result<&str, _>| Ok(s?.parse()?))
                    .collect::<AocResult<_>>()?,
            ))
        })
        .collect()
}

fn floodfill(connections: &Connections, start: usize, value: i32, field: &mut [i32]) {
    let mut to_fill = connections[&start].clone();
    while let Some(tgt) = to_fill.pop() {
        if field[tgt] != value {
            field[tgt] = value;
            to_fill.extend(&connections[&tgt]);
        }
    }
}

pub fn task1(input: &Connections) -> AocResult<usize> {
    let mut zero_connected = vec![-1; input.len()];
    floodfill(input, 0, 0, &mut zero_connected);
    Ok(zero_connected.iter().filter(|&&x| x == 0).count())
}

pub fn task2(input: &Connections) -> AocResult<i32> {
    let mut connect: Vec<i32> = vec![-1; input.len()];
    let mut group_index = 0;
    while let Some((first, _x)) = connect.iter().enumerate().find(|&(_, &x)| x == -1) {
        floodfill(input, first, group_index, &mut connect);
        group_index += 1;
    }
    Ok(group_index)
}
