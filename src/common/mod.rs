const AOC_INI_FILE_NAME: &str = "aoc.ini";
const AOC_INI_SECTION: &str = "aoc";

macro_rules! log {
    ($($arg : tt) *) =>
    (
        if cfg!(feature="verbose") {
            println!($($arg) *);
        } else if cfg!(feature="err") {
            eprintln!($($arg) *);
        }
    )
}

fn format_url(format: &str, year: &str, number: &str) -> String {
    format.replacen("{}", year, 1).replacen("{}", number, 1)
}

#[derive(Debug)]
pub enum Error {
    MinreqError(minreq::Error),
    WrongResponce(String),
    IniError(ini::Error),
    WrongIniFormat,
}

type Result<T> = std::result::Result<T, Error>;

impl From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Error {
        Error::MinreqError(err)
    }
}

impl From<ini::Error> for Error {
    fn from(err: ini::Error) -> Error {
        Error::IniError(err)
    }
}

pub fn get_input(
    url: &str,
    session: &str,
) -> std::result::Result<String, Error> {
    log!("Trying url '{}'", url);
    let resp = minreq::get(url)
        .with_header("Cookie", format!("session={}", session))
        .send()?;

    if 200 <= resp.status_code && resp.status_code < 300 {
        let result = resp.as_str()?;
        Ok(result[..result.len() - 1].to_owned())
    } else {
        Err(Error::WrongResponce(resp.reason_phrase))
    }
}

pub fn get_input_from_ini_with_mod(
    mod_year: &str,
    mod_day: &str,
) -> Result<String> {
    get_input_from_ini_with_year(&mod_year[4..], &mod_day[3..])
}

pub fn get_input_from_ini_with_year(year: &str, day: &str) -> Result<String> {
    let filename = format!("cache{}_{}.txt", year, day);
    std::fs::read_to_string(&filename).or_else(|file_error| {
        log!("Cache not found ({})", file_error);
        let number: String = day.chars().filter(|c| c.is_digit(10)).collect();
        let ini = ini::Ini::load_from_file(AOC_INI_FILE_NAME)?;
        let section = ini
            .section(Some(AOC_INI_SECTION))
            .ok_or(Error::WrongIniFormat)?;
        let url = format_url(
            section.get("link_year").ok_or(Error::WrongIniFormat)?,
            year,
            &number,
        );
        let session = &section["session"];
        get_input(&url, session).and_then(|s| {
            if let Err(e) = std::fs::write(filename, &s) {
                log!("{:?}", e);
            }
            Ok(s)
        })
    })
}
