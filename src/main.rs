pub mod common;

mod year2015;
mod year2016;
mod year2019;
mod year2020;
mod year2021;
mod year2022;

use clap::Parser;

#[derive(Parser)]
struct Args {
    year: String,

    day: String,
}

fn main() {
    let args = Args::parse();
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    type Fun = fn(&str);
    let tasks = Lazy::<HashMap<String, Fun>>::new(|| {
        [
            ("2015".to_owned(), year2015::task as Fun),
            ("2016".to_owned(), year2016::task),
            ("2019".to_owned(), year2019::task),
            ("2020".to_owned(), year2020::task),
            ("2021".to_owned(), year2021::task),
            ("2022".to_owned(), year2022::task),
        ]
        .into_iter()
        .collect()
    });

    let start = std::time::Instant::now();

    tasks[&args.year](&format!("day{}", args.day));

    println!("Time elapsed: {:?}ms", start.elapsed().as_millis());
}
