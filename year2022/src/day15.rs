use crate::*;

type Point = (i32, i32);
type SensorData = (Point, Point);

pub fn parse_input(input: &str) -> AocResult<Vec<SensorData>> {
    input
        .lines()
        .map(|line| {
            let (x, y, bx, by) = prse::try_parse!(
                line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i32,
                i32,
                i32,
                i32
            )?;
            Ok(((x, y), (bx, by)))
        })
        .collect()
}

fn generate_row(sensors: &[SensorData], row_y: i32) -> Vec<(i32, i32)> {
    let mut list: Vec<_> = sensors
        .iter()
        .filter_map(|((x, y), (bx, by))| {
            let dist = (bx - x).abs() + (by - y).abs();
            let dy = dist - (row_y - y).abs();
            if dy >= 0 {
                Some((x - dy, x + dy))
            } else {
                None
            }
        })
        .collect();
    list.sort_by_key(|&(left, _right)| left);
    let mut i = 0;
    while i < list.len() - 1 {
        if list[i + 1].0 <= list[i].1 + 1 {
            list[i].1 = std::cmp::max(list[i].1, list[i + 1].1);
            list.remove(i + 1);
        } else {
            i += 1;
        }
    }
    list
}

pub fn task1(sensors: &[SensorData]) -> AocResult<i32> {
    const ROW_Y: i32 = 2_000_000;
    let list = generate_row(sensors, ROW_Y);

    let mut beacons: Vec<_> = sensors
        .iter()
        .filter_map(|&(_, (bx, by))| if by == ROW_Y { Some(bx) } else { None })
        .collect();
    beacons.sort();
    beacons.dedup();

    Ok(list
        .iter()
        .map(|(left, right)| right - left + 1)
        .sum::<i32>()
        - beacons.len() as i32)
}

pub fn task2(sensors: &[SensorData]) -> AocResult<usize> {
    const SIZE: i32 = 4_000_000;
    for row_y in 0..=SIZE {
        let mut list = generate_row(sensors, row_y);
        loop {
            let last = list.len() - 1;
            if list[last].0 > SIZE {
                list.remove(last);
            } else if list[last].1 > SIZE {
                list[last].1 = SIZE;
                break;
            } else {
                break;
            }
        }
        loop {
            if list[0].1 < 0 {
                list.remove(0);
            } else if list[0].0 < 0 {
                list[0].0 = 0;
                break;
            } else {
                break;
            }
        }
        if list.len() == 1 {
            if list[0].0 > 0 {
                return Ok(row_y as usize);
            } else if list[0].1 < SIZE {
                return Ok(SIZE as usize * SIZE as usize + row_y as usize);
            }
        } else {
            return Ok((list[0].1 as usize + 1) * SIZE as usize + row_y as usize);
        }
    }
    Err(aoc_error!("Not found"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = [
            ((0, 2_000_000), (-1, 2_000_000)),
            ((3, 2_000_001), (5, 2_000_001)),
        ];
        assert_eq!(task1(&input).unwrap(), 5);
    }
}
