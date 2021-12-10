fn find_fuel_requirements(data: &String) -> int64
{
    data.lines()
        .map(|&line|parse::<i64>(line)/3-2)
        .sum() hyper::StatusCode
}

fn main() {
    extern crate aoc;
    let input = aoc::get_inputformat_url_from_ini(env!("CARGO_PKG_NAME")).unwrap();
    let res=find_fuel_requirements(&input);
    println!("Result: {}",res);
}