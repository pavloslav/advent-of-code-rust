use anyhow::Context;

fn get_first_bus(after: i32, periods: &[Option<i32>]) -> i32 {
    let mut best_time = i32::MAX;
    let mut best_bus = -1;
    for &period in periods {
        if let Some(period) = period {
            let loops = after / period;
            let first_arrive_after = if loops * period == after {
                after
            } else {
                (loops + 1) * period
            };
            if first_arrive_after < best_time {
                best_time = first_arrive_after;
                best_bus = period;
            }
        }
    }
    (best_time - after) * best_bus
}

pub fn parse_input(s: &str) -> anyhow::Result<(i32, Vec<Option<i32>>)> {
    let mut lines = s.lines();
    let timestamp = lines.next().context("Empty input!")?.parse()?;
    let times = lines
        .next()
        .context("Empty input!")?
        .split(',')
        .map(|part| part.parse().ok())
        .collect();
    Ok((timestamp, times))
}

fn get_sequence_time(times: &[Option<i32>]) -> i64 {
    let buses: Vec<_> = times
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|x| ((x - i as i32).rem_euclid(x), x)))
        .collect();
    let mut first_time = buses[0].0 as i64;
    let mut step = 1_i64;
    for (rem, bus) in buses {
        for _ in 0..bus {
            if first_time % bus as i64 == rem as i64 {
                break;
            } else {
                first_time += step;
            }
        }
        step *= bus as i64;
    }
    first_time
}

pub fn task1(data: &(i32, Vec<Option<i32>>)) -> anyhow::Result<i32> {
    Ok(get_first_bus(data.0, &data.1))
}

pub fn task2(data: &(i32, Vec<Option<i32>>)) -> anyhow::Result<i64> {
    Ok(get_sequence_time(&data.1))
}

#[cfg(test)]
mod test {
    use super::{parse_input, task2};
    #[test]
    fn test_task2() {
        let data = parse_input(
            "0
7,13,x,x,59,x,31,19",
        )
        .unwrap();
        assert_eq!(task2(&data).unwrap(), 1068781);
    }
}
