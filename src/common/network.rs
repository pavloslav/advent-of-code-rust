/*
 * Wrapper for minreq crate
 */

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] minreq::Error),
    #[error("Server responded with fail: {0}")]
    WrongResponce(String),
}

pub fn get_input_from_url(
    url: &str,
    session: &str,
) -> std::result::Result<String, Error> {
    let resp = minreq::get(url)
        .with_header("Cookie", format!("session={session}",))
        .send()?;

    if 200 <= resp.status_code && resp.status_code < 300 {
        let result = resp.as_str()?;
        Ok(result.to_owned())
    } else {
        Err(Error::WrongResponce(resp.reason_phrase))
    }
}
