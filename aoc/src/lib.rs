extern crate hyper;
extern crate ini;

use std::io::Read;
use ini::Ini;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::error::Error),
    NotOk(hyper::status::StatusCode),
    IO(std::io::Error),
    Ini(ini::ini::Error), 
    NoAocSectionInIni,
} 

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Hyper(ref err)    => {write!(f,"Advent of Code net error: {}",err)},
            Error::NotOk(ref code)   => {write!(f,"Advent of Code recieved result {}",code)},
            Error::IO(ref err)       => {write!(f,"Advent of Code couldn't read because {}",err)},
            Error::Ini(ref err)      => {write!(f,"Advent of Code failed to read ini because {}",err)},
            Error::NoAocSectionInIni => {write!(f,"Advent of Code failed to read ini because no [aoc] section in INI file")}, 
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref err) => err.description(),
            Error::NotOk(_)       => "Advent of Code recieved wrong result downloading input",
            Error::IO(ref err)    => err.description(),
            Error::Ini(ref err)   => err.description(),
            Error::NoAocSectionInIni => "No [aoc] section in INI file",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }    
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<ini::ini::Error> for Error {
    fn from(err: ini::ini::Error) -> Error {
        Error::Ini(err)
    }
}

use hyper::header::{Cookie, CookiePair};

pub fn get_input(url:&str, number:&str, session:&str) -> Result<String, Error> {
    let url=str::replace(url,"{}", number);
    println!("{}",url);
    let cookie = Cookie(
                        vec![ 
                            CookiePair::new("session".to_string(),
                                              session.to_string() ) 
                            ]
                    );
    println!("Trying url {}",url);
    let mut resp = hyper::Client::new()
                .get(&url)
                .header(cookie)
                .send()?;

    match resp.status {
        hyper::Ok => {    
            let mut answ:String = String::new();
            resp.read_to_string(&mut answ)?;
            let len = answ.len()-1;
            answ.truncate(len);
            Ok(answ)
        },
        _ => {
            Err(Error::NotOk(resp.status))
        }
    }
}

pub fn get_input_from_ini(number:&str) -> Result<String, Error> {
    let ini = Ini::load_from_file("..\\aoc.ini")?;
    let section = ini.section(Some("aoc"));
    match section {
        Some(section) => {
            let url     = &section["link".into()];
            let session = &section["session".into()];
            get_input(url, number, session)
        },
        None => Err(Error::NoAocSectionInIni)
    }
}