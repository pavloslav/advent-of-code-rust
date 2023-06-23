#[derive(thiserror::Error, Debug)]
pub enum SettingsError {
    #[error("Problem in file: {0}")]
    File(#[from] std::io::Error),
    #[error("Incorrect data in file: {0}")]
    Json(#[from] serde_json::error::Error),
}

pub fn read_setting(file_name: &str) -> Result<super::Settings, SettingsError> {
    let file = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(file);
    let settings = serde_json::from_reader(reader)?;
    Ok(settings)
}
