use std::fmt::Display;
use std::time::Duration;
use std::time::SystemTime;
use std::time::SystemTimeError;

use super::network;
use super::settings;

const AOC_SETTINGS_FILE_NAME: &str = "aoc.ini";

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

#[derive(Debug)]
pub enum Error {
    Network(network::Error),
    Settings(settings::Error),
    TimeError(SystemTimeError),
    WrongTask,
    TaskError(String),
    Clap(clap::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<network::Error> for Error {
    fn from(err: network::Error) -> Error {
        Error::Network(err)
    }
}

impl From<settings::Error> for Error {
    fn from(err: settings::Error) -> Error {
        Error::Settings(err)
    }
}

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Error {
        Error::TimeError(err)
    }
}

impl From<clap::error::Error> for Error {
    fn from(err: clap::error::Error) -> Error {
        Error::Clap(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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
        let settings = settings::read_setting(AOC_SETTINGS_FILE_NAME)?;
        let url = settings.format_url(year, day);
        log!("Trying url '{}'", url);
        network::get_input_from_url(&url, &settings.session)
            .map(|s| {
                if let Err(e) = std::fs::write(filename, &s) {
                    log!("{:?}", e);
                }
                s
            })
            .map_err(Error::from)
    })
}

#[derive(Clone, Copy)]
pub struct FunctionHolderPanic {
    pub f: fn(),
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! mod_list_panic {
    ($year: ident, $($day: ident),+) => {
        use once_cell::sync::OnceCell;
        $(pub mod $day;)*
        pub fn task(day: &str) {
            let fn_map = FN_MAP.get_or_init(||std::collections::HashMap::from ([
                $((stringify!($day), crate::common::FunctionHolderPanic {
                        f: || {
                            let year_str = stringify!($year);
                            let day_str = stringify!($day);
                            let input =
                                crate::common::get_input_with_mod(year_str, day_str)
                                    .unwrap();
                            let data = $day::parse_input(&input);
                            println!("{} {}", year_str, day_str);
                            println!("Result 1:\n{}", $day::task1(&data));
                            println!("Result 2:\n{}", $day::task2(&data));
                        },
                    }),)*
                ]));
            (fn_map[day].f)()

        }
        static FN_MAP : OnceCell<std::collections::HashMap<&'static str, crate::common::FunctionHolderPanic>> = OnceCell::new();

    }
}

#[derive(Clone, Copy)]
pub struct FunctionHolder {
    pub f: fn() -> Result<()>,
}

pub fn measure<T, F>(
    call: F,
) -> (T, std::result::Result<Duration, std::time::SystemTimeError>)
where
    F: FnOnce() -> T,
{
    let now = SystemTime::now();
    (call(), now.elapsed())
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! mod_list {
    ($year: ident, $($day: ident),+) => {
        use once_cell::sync::Lazy;
        use super::common::aoc::Result;
        use super::common::aoc::Error;
        use super::common::aoc::measure;

        $(pub mod $day;)*

        pub fn task(day: &str) -> Result<()> {
            (FN_MAP.get(day).ok_or(Error::WrongTask)?.f)()
        }

        static FN_MAP : Lazy<std::collections::HashMap<&'static str, crate::common::FunctionHolder>>
            = Lazy::new(||std::collections::HashMap::from ([
                $((stringify!($day), crate::common::FunctionHolder {
                        f: || {
                            let year_str = stringify!($year);
                            let day_str = stringify!($day);
                            let input =
                                crate::common::get_input_with_mod(year_str, day_str)
                                    .unwrap();
                            println!("{} {}", year_str, day_str);

                            let (data, time_parse) = measure(||$day::parse_input(&input));
                            let data = data?;
                            println!("Parsing time {}s", time_parse?.as_secs_f64());

                            let (result1, time_task1) = measure(||$day::task1(&data));
                            println!("Result 1:\n{}\nTime: {}s", result1?, time_task1?.as_secs_f64());

                            let (result2, time_task2) = measure(||$day::task2(&data));
                            println!("Result 2:\n{}\nTime: {}s", result2?, time_task2?.as_secs_f64());

                            Ok(())
                        },
                    }),)*
                ]));

    }
}