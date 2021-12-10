fn get_first_bus(after:i32, periods:&Vec<i32>) -> i32
{
    let mut best_time = i32::MAX;
    let mut best_bus = -1;
    for &period in periods {
        if period>0 {
            let loops = after/period;
            let first_arrive_after = if loops*period==after {after} else {(loops+1)*period};
            if first_arrive_after<best_time {
                best_time = first_arrive_after;
                best_bus = period;
            }
        }
    }
    (best_time-after)*best_bus
}

fn parse_input(s:&str) -> (i32, Vec<i32>)
{
    let mut lines = s.lines();
    let timestamp = lines.next()
                         .unwrap()
                         .parse()
                         .unwrap();
    let times = lines.next()
                     .unwrap()
                     .split(',')
                     .map(|part|part.parse().unwrap_or(-1))
                     .collect();
    (timestamp, times)
}

fn get_sequence_time(times:&Vec<i32>) -> i64
{
    let buses:Vec<_> = times.iter()
                            .enumerate()
                            .filter_map(|(i,&x)|if x>0 {Some(((x-i as i32).rem_euclid(x), x))} else {None})
                            .collect();
    let mut first_time = buses[0].0 as i64;
    let mut step = 1_i64; 
    for (rem, bus) in buses {
        for _ in 0..bus {
            if first_time%bus as i64 == rem as i64 {
                break;
            } else {
                first_time += step;
            }
        }
        step *= bus as i64;
    }
    first_time
}

fn task1(s:&str) -> i32
{
    let (timestamp, times) = parse_input(s);
    get_first_bus(timestamp, &times)
}

fn task2(s:&str) -> i64
{
    let (_, times) = parse_input(s);
    get_sequence_time(&times)
}

#[cfg(test)]
mod test {
    use crate::task2;
    #[test]
    fn test_task2()
    {
        assert_eq!(task2("0
7,13,x,x,59,x,31,19"),1068781);
    }
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("13","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}