use common::Error;
use common::Result;

use clap::Parser;
use common::Args;

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
            return Err(Error::WrongTask {
                year: year.to_owned(),
                day: args.day.clone(),
            });
        }
    })(&format!("day{}", args.day))
}
