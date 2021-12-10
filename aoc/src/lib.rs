//extern crate reqwest;
extern crate ini;
extern crate anyhow;

use ini::Ini;
//use reqwest::blocking::*;
//use reqwest::header::*;
use std::fs;

fn format_url(format: &str, year: &str, number: &str) -> String {
    format.replacen("{}", year, 1).replacen("{}", number, 1)
}
/*
pub fn get_input(url:&str, session:&str) -> anyhow::Result<String> {
    println!("Session: '{}'",session);
    println!("Trying url '{}'",url);
    let client = Client::new();
    let resp = client.get(url)
        .header(COOKIE, format!("session={}",session))
        //.header(USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
       // .body("\r\n\r\n")
        .send()?;

    if resp.status().is_success() {
        let result = resp.text()?;
        let len = result.len()-1;
        Ok(result[..len].to_owned())
    } else {
        anyhow::bail!(resp.status())
    }
}*/

extern crate minreq;

pub fn get_input(url:&str, session:&str) -> anyhow::Result<String> {
    println!("Trying url '{}'",url);
    let resp = minreq::get(url)
                      .with_header("Cookie", format!("session={}",session))
                      .send()?;

    if 200 <= resp.status_code && resp.status_code <300 {
        let result = resp.as_str()?;
        Ok(result[..result.len()-1].to_owned())
    } else {
        anyhow::bail!(resp.reason_phrase)
    }
}

pub fn get_input_from_ini(number:&str) -> anyhow::Result<String> {
    get_input_from_ini_with_year(number, &"2016")
}

pub fn get_input_from_ini_with_year(number:&str, year:&str) -> anyhow::Result<String> {
    let filename = format!("cache{}_{}.txt",year, number);
    fs::read_to_string(&filename).or_else(|file_error|
    {
        println!("Cache not found ({})", file_error);
        let number = match number.find('_') {
            Some(idx) => &number[idx+1..],
            _ => number
        };
        let ini = Ini::load_from_file("..\\aoc.ini")?;
        let section = ini.section(Some("aoc"));
        match section {
            Some(section) => {
                let url     = format_url(&section["link_year"], year, number);
                let session = &section["session"];
                get_input(&url, session).and_then(|s|{
                    if let Err(e) = fs::write(filename, &s) {
                        println!("{:?}",e);
                    }
                    Ok(s)
                })
            },
            None => anyhow::bail!("No aoc section in ini")
        }
    })
}