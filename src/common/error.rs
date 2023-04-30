#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    TaskError(String),
    #[error("{0}")]
    Network(#[from] super::network::Error),
    #[error("{0}")]
    Settings(#[from] super::settings::SettingsError),
    #[error("TimeError: {0}")]
    TimeError(#[from] std::time::SystemTimeError),
    #[error("Clap error: {0}")]
    Clap(#[from] clap::error::Error),
    #[error("Error parsing int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Error parsing char: {0}")]
    ParseChar(#[from] std::char::ParseCharError),
    #[error("Error in ScanFmt: {0}")]
    ScanFmt(#[from] scan_fmt::parse::ScanError),
    #[error("Serde JSON Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Fancy Regex error: {0}")]
    FancyRegex(#[from] fancy_regex::Error),
    #[error("Incorrect task: year {year}, day {day}")]
    WrongTask { year: String, day: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! task_error {
    ($($arg:tt)*) => {
        Error::TaskError(format!("{} line {}: {}", file!(), line!(), format!($($arg)*)))
    };
}
