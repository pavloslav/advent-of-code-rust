use std::fmt::Display;
use std::time::SystemTimeError;

#[derive(Debug)]
pub enum Error {
    Network(super::network::Error),
    Settings(super::settings::Error),
    TimeError(SystemTimeError),
    Clap(clap::error::Error),
    ParseInt(std::num::ParseIntError),
    ParseChar(std::char::ParseCharError),
    ScanFmt(scan_fmt::parse::ScanError),
    SerdeJson(serde_json::Error),
    TaskError(String),
    WrongTask,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<super::network::Error> for Error {
    fn from(err: super::network::Error) -> Error {
        Error::Network(err)
    }
}

impl From<super::settings::Error> for Error {
    fn from(err: super::settings::Error) -> Error {
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

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<std::char::ParseCharError> for Error {
    fn from(err: std::char::ParseCharError) -> Error {
        Error::ParseChar(err)
    }
}

impl From<scan_fmt::parse::ScanError> for Error {
    fn from(err: scan_fmt::parse::ScanError) -> Error {
        Error::ScanFmt(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AoC Error: {self:?}")
    }
}
