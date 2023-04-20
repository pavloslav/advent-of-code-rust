pub mod common;

use common::Result;

mod year2015;
mod year2016;
mod year2017;
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
        ("2019".to_owned(), year2019::task),
        //("2020".to_owned(), year2020::task),
        //("2021".to_owned(), year2021::task),
        //("2022".to_owned(), year2022::task),
    ]
    .into_iter()
    .collect();

    type FunPanic = fn(&str);
    let _tasks_panic: HashMap<String, FunPanic> = [
        //("2017".to_owned(), year2017::task as FunPanic),
        //("2019".to_owned(), year2019::task as FunPanic),
        ("2020".to_owned(), year2020::task as FunPanic),
        ("2021".to_owned(), year2021::task as FunPanic),
        ("2022".to_owned(), year2022::task as FunPanic),
    ]
    .into_iter()
    .collect();

    tasks
        .get(&args.year)
        .unwrap_or_else(|| panic!("Year {} is incorrect!", args.year))(
        &format!("day{}", args.day),
    )
}
