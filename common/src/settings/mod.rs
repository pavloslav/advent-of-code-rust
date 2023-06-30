/*
 * Common interface for ini and json
 */

//pub mod ini;
pub mod json;

//const AOC_SETTINGS_FILE_NAME_INI: &str = "aoc.ini";
const AOC_SETTINGS_FILE_NAME_JSON: &str = "settings.json";

#[derive(serde::Deserialize)]
pub struct Settings {
    pub session: String,
    pub link_year: String,
}

impl Settings {
    pub fn format_url(&self, year: &str, day: &str) -> String {
        self.link_year.replace("{year}", year).replace("{day}", day)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
    #[error("Problem in file: {0}")]
    //Ini(#[from] ini::SettingsError),
    //#[error("Incorrect data in file: {0}")]
    Json(#[from] json::SettingsError),
    #[error("Problem deserializing: {0}")]
    File(#[from] std::io::Error),
}

pub fn read_setting() -> Result<Settings, SettingsError> {
    Ok(json::read_setting(AOC_SETTINGS_FILE_NAME_JSON)?)
}
