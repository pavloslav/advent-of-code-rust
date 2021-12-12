mod common;

mod year2015;
mod year2016;
mod year2019;
mod year2020;
mod year2021;

struct FunctionHolder {
    f: fn(),
}

#[allow(dead_code)]
macro_rules! aoc_entry_part {
    ($year: ident, $day: ident) => {
        ((stringify!($year), stringify!($day)), FunctionHolder{ f:|| {
            let year_str = stringify!($year);
            let day_str  = stringify!($day);
            let input = common::get_input_from_ini_with_mod(year_str, day_str).unwrap();
            let data = $year::$day::parse_input(&input);
            println!("{} {}",year_str, day_str);
            println!("Result 1: {}", $year::$day::task1(&data));
        }})
    }
}

macro_rules! aoc_entry {
    ($year: ident, $day: ident) => {
        ((stringify!($year), stringify!($day)), FunctionHolder{ f:|| {
            let year_str = stringify!($year);
            let day_str  = stringify!($day);
            let input = common::get_input_from_ini_with_mod(year_str, day_str).unwrap();
            let data = $year::$day::parse_input(&input);
            println!("{} {}",year_str, day_str);
            println!("Result 1: {}", $year::$day::task1(&data));
            println!("Result 2: {}", $year::$day::task2(&data));
        }})
    }
}

fn main() {
    let aoc_map = std::collections::HashMap::from([
        aoc_entry!(year2015, day1), // wrong first answer, dunno
        aoc_entry!(year2015, day2),
        aoc_entry!(year2015, day3),
        aoc_entry!(year2015, day4),
        aoc_entry!(year2015, day5),
        aoc_entry!(year2015, day6),
        aoc_entry!(year2015, day7),
        aoc_entry!(year2015, day8),
        aoc_entry!(year2015, day9),
        aoc_entry!(year2015, day10),
        aoc_entry!(year2015, day11),
        aoc_entry!(year2015, day12),

        aoc_entry!(year2016, day1),
        aoc_entry!(year2016, day2),
        aoc_entry!(year2016, day3),
        aoc_entry!(year2016, day4),
        aoc_entry!(year2016, day5),
        aoc_entry!(year2016, day6),
        aoc_entry!(year2016, day7),
        aoc_entry_part!(year2016, day8),
        aoc_entry!(year2016, day9),
//        aoc_entry!(year2016, day10),

        aoc_entry_part!(year2019, day1),

        aoc_entry!(year2020, day1a),
        aoc_entry!(year2020, day2),
        aoc_entry!(year2020, day3),
        aoc_entry!(year2020, day4),
        aoc_entry!(year2020, day5),
        aoc_entry!(year2020, day6),
        aoc_entry!(year2020, day7),
        aoc_entry!(year2020, day8),
        aoc_entry!(year2020, day9),
        aoc_entry!(year2020, day10),
        aoc_entry!(year2020, day11),
        aoc_entry!(year2020, day12),
        aoc_entry!(year2020, day13),
        aoc_entry!(year2020, day14),
        aoc_entry!(year2020, day15),
        aoc_entry!(year2020, day16),
        aoc_entry!(year2020, day17),
        aoc_entry!(year2020, day18),
        aoc_entry_part!(year2020, day19),

        aoc_entry!(year2021, day1),
        aoc_entry!(year2021, day2),
        aoc_entry!(year2021, day3),
        aoc_entry!(year2021, day4),
        aoc_entry!(year2021, day5),
        aoc_entry!(year2021, day5a),
        aoc_entry!(year2021, day6),
        aoc_entry!(year2021, day7),
        aoc_entry!(year2021, day9),
        aoc_entry!(year2021, day10),
        aoc_entry!(year2021, day11),
        aoc_entry!(year2021, day12),

    ]);
    
    macro_rules! call {
        ($year: ident, $day: ident) => {
            (aoc_map[&(stringify!($year), stringify!($day))].f)();
        }
    }

    call!(year2015, day12);


}
