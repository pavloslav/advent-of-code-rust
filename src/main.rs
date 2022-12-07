pub mod common;

#[allow(dead_code)]
mod year2015;
#[allow(dead_code)]
mod year2016;
#[allow(dead_code)]
mod year2019;
#[allow(dead_code)]
mod year2020;
#[allow(dead_code)]
mod year2021;
#[allow(dead_code)]
mod year2022;

fn main() {
    let start = std::time::Instant::now();

    year2022::task("day7");

    println!("Time elapsed: {:?}ms", start.elapsed().as_millis());
}
