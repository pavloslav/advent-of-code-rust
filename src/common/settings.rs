/*
 * Wrapper for ini crate
 */

const AOC_INI_SECTION: &str = "aoc";
const INI_SESSION_NAME: &str = "session";
const INI_LINK_YEAR_NAME: &str = "link_year";

pub struct Settings {
    pub session: String,
    pub format_url_string: String,
}

impl Settings {
    pub fn format_url(&self, year: &str, day: &str) -> String {
        self.format_url_string
            .replace("{year}", year)
            .replace("{day}", day)
    }
}

#[derive(Debug)]
pub enum Error {
    Ini(ini::Error),
    Aoc(String),
}

impl From<ini::Error> for Error {
    fn from(err: ini::Error) -> Error {
        Error::Ini(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

fn read_value<'a>(section: &'a ini::Properties, name: &str) -> Result<&'a str> {
    section
        .get(name)
        .ok_or(Error::Aoc(format!("Value {} not found in ini file", name)))
}

pub fn read_setting(file_name: &str) -> Result<Settings> {
    let ini = ini::Ini::load_from_file(file_name)?;
    let section = ini
        .section(Some(AOC_INI_SECTION))
        .ok_or(Error::Aoc(format!("No section {} found", AOC_INI_SECTION)))?;
    Ok(Settings {
        session: read_value(section, INI_SESSION_NAME)?.to_string(),
        format_url_string: read_value(section, INI_LINK_YEAR_NAME)?.to_string(),
    })
}
