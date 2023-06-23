use aoc_common::aoc_error;
use aoc_common::Error;
use aoc_common::Result;

mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;
mod year2021;
mod year2022;

use aoc_common::Args;
use clap::Parser;

pub fn exec() -> Result<()> {
    let args = Args::try_parse()?;
    type Fun = fn(&str) -> Result<()>;
    (match args.year.as_str() {
        "2015" => year2015::task as Fun,
        "2016" => year2016::task as Fun,
        "2017" => year2017::task as Fun,
        "2018" => year2018::task as Fun,
        "2019" => year2019::task as Fun,
        "2020" => year2020::task as Fun,
        "2021" => year2021::task as Fun,
        "2022" => year2022::task as Fun,
        year => {
            println!("---{}---", args.year.as_str());
            return Err(Error::WrongTask {
                year: year.to_owned(),
                day: args.day.clone(),
            });
        }
    })(&format!("day{}", args.day))
}
