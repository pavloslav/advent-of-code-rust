/*
 * Wrapper for minreq crate
 */

#[derive(Debug)]
pub enum Error {
    Network(minreq::Error),
    WrongResponce(String),
}

impl From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Error {
        Error::Network(err)
    }
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
