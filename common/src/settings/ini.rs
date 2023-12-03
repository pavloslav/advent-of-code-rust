/*
 * Wrapper for ini crate
 */

const AOC_INI_SECTION: &str = "aoc";
const INI_SESSION_NAME: &str = "session";
const INI_LINK_YEAR_NAME: &str = "link_year";

#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
    #[error("Problem in file: {0}")]
    Ini(#[from] ::ini::Error),
    #[error("Incorrect data in file: {0}")]
    Aoc(String),
}

fn read_value<'a>(section: &'a ini::Properties, name: &str) -> AocResult<&'a str, SettingsError> {
    section.get(name).ok_or(SettingsError::Aoc(format!(
        "Value {} not found in ini file",
        name
    )))
}

pub fn read_setting(file_name: &str) -> AocResult<super::Settings, SettingsError> {
    let ini = ini::Ini::load_from_file(file_name)?;
    let section = ini
        .section(Some(AOC_INI_SECTION))
        .ok_or(SettingsError::Aoc(format!(
            "No section {} found",
            AOC_INI_SECTION
        )))?;
    Ok(super::Settings {
        session: read_value(section, INI_SESSION_NAME)?.to_string(),
        link_year: read_value(section, INI_LINK_YEAR_NAME)?.to_string(),
    })
}
