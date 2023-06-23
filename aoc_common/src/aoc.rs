use super::error::Result;
use std::time::Duration;
use std::time::SystemTime;

use super::network;
use super::settings;

#[cfg(feature = "verbose")]
macro_rules! log {
    ($($arg : tt) *) =>
    (
        println!($($arg) *);
    )
}

#[cfg(feature = "err")]
macro_rules! log {
    ($($arg : tt) *) =>
    (
        eprintln!($($arg) *);
    )
}

pub fn get_input_with_mod(mod_year: &str, mod_day: &str) -> Result<String> {
    get_input(&mod_year[4..], &mod_day[3..])
}

fn cache_file_name(year: &str, day: &str) -> String {
    format!("cache/cache{}_{}.txt", year, day)
}

pub fn get_input(year: &str, day: &str) -> Result<String> {
    let filename = cache_file_name(year, day);
    std::fs::read_to_string(&filename).or_else(|file_error| -> Result<String> {
        log!("Cache not found ({})", file_error);
        let settings = settings::read_setting()?;
        let url = settings.format_url(year, day);
        log!("Trying url '{}'", url);

        Ok(
            network::get_input_from_url(&url, &settings.session).map(|s| {
                if let Err(e) = std::fs::write(filename, &s) {
                    log!("{:?}", e);
                }
                s
            })?,
        )
    })
}

#[derive(Clone, Copy)]
pub struct FunctionHolder {
    pub f: fn() -> Result<()>,
}

pub fn measure<T, F>(call: F) -> (T, std::result::Result<Duration, std::time::SystemTimeError>)
where
    F: FnOnce() -> T,
{
    let now = SystemTime::now();
    (call(), now.elapsed())
}

#[macro_export]
//#[allow(clippy::crate_in_macro_def)]
macro_rules! mod_list {
    ($year: ident, $($day: ident),+) => {
        use once_cell::sync::Lazy;
        //use $crate::measure;

        $(pub mod $day;)*

        pub fn task(day: &str) -> $crate::Result<()> {
            (FN_MAP.get(day).ok_or($crate::Error::WrongTask{year:stringify!($year).to_string(), day:day.to_string()})?.f)()
        }

        static FN_MAP : Lazy<std::collections::HashMap<&'static str, $crate::FunctionHolder>>
            = Lazy::new(||std::collections::HashMap::from ([
                $((stringify!($day), $crate::FunctionHolder {
                        f: || {
                            let year_str = stringify!($year);
                            let day_str = stringify!($day);
                            let input =
                                $crate::get_input_with_mod(year_str, day_str)?;
                            println!("{} {}", year_str, day_str);

                            let (data, time_parse) = $crate::aoc::measure(||$day::parse_input(&input));
                            let data = data?;
                            println!("Parsing time {}s", time_parse?.as_secs_f64());

                            let (result1, time_task1) = $crate::aoc::measure(||$day::task1(&data));
                            println!("Result 1:\n{}\nTime: {}s", result1?, time_task1?.as_secs_f64());

                            let (result2, time_task2) = $crate::aoc::measure(||$day::task2(&data));
                            println!("Result 2:\n{}\nTime: {}s", result2?, time_task2?.as_secs_f64());

                            Ok(())
                        },
                    }),)*
                ]));

    }
}
