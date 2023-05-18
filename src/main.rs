pub mod common;

use common::Error;
use common::Result;

mod year2015;
mod year2016;
mod year2017;
mod year2018;
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
    main_unsafe().unwrap_or_else(|err| {
        println!("{err}");
    });
}

fn main_unsafe() -> Result<()> {
    let args = Args::try_parse()?;
    use std::collections::HashMap;
    type Fun = fn(&str) -> Result<()>;
    let tasks: HashMap<String, Fun> = [
        ("2015".to_owned(), year2015::task as Fun),
        ("2016".to_owned(), year2016::task),
        ("2017".to_owned(), year2017::task),
        ("2018".to_owned(), year2018::task),
        ("2019".to_owned(), year2019::task),
        ("2020".to_owned(), year2020::task),
        ("2021".to_owned(), year2021::task),
        ("2022".to_owned(), year2022::task),
    ]
    .into_iter()
    .collect();

    tasks.get(&args.year).ok_or_else(|| Error::WrongTask {
        year: args.year.clone(),
        day: args.day.clone(),
    })?(&format!("day{}", args.day))
}
