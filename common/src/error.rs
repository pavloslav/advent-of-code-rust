#[derive(thiserror::Error, Debug)]
pub enum AocError {
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
    #[error("Serde JSON Error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("Error parsing float: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Fancy Regex error: {0}")]
    FancyRegex(#[from] fancy_regex::Error),
    #[error("Incorrect task: year '{year}', day '{day}'")]
    WrongTask { year: String, day: String },
    #[error("Convertion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Psre error: {0}")]
    PsreError(#[from] prse::ParseError),
}

pub type AocResult<T> = std::result::Result<T, AocError>;

#[macro_export]
macro_rules! aoc_error {
    ($($arg:tt)*) => {
        AocError::TaskError(format!("{}:{}: {}", file!(), line!(), format!($($arg)*)))
    };
}
