use clap::Parser;
use common::Args;

pub fn exec() -> anyhow::Result<()> {
    let args = Args::try_parse()?;
    type Fun = fn(&str) -> anyhow::Result<()>;
    (match args.year.as_str() {
        "2015" => year2015::task as Fun,
        "2016" => year2016::task as Fun,
        "2017" => year2017::task as Fun,
        "2018" => year2018::task as Fun,
        "2019" => year2019::task as Fun,
        "2020" => year2020::task as Fun,
        "2021" => year2021::task as Fun,
        "2022" => year2022::task as Fun,
        "2023" => year2023::task as Fun,
        "2024" => year2024::task as Fun,
        year => anyhow::bail!("Incorrect task: year '{year}'"),
    })(&format!("day{}", args.day))
}
