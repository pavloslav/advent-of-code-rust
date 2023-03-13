const AOC_INI_FILE_NAME: &str = "aoc.ini";
const AOC_INI_SECTION: &str = "aoc";

#[cfg(feature = "verbose")]
macro_rules! log {
    ($($arg : tt) *) =>
    (
        println!($($arg) *);
    )
}

#[cfg(feature = "err")]
macro_rules! log {
    ($($arg : tt) *) =>
    (
        eprintln!($($arg) *);
    )
}

fn format_url(format: &str, year: &str, number: &str) -> String {
    format.replacen("{}", year, 1).replacen("{}", number, 1)
}

#[derive(Debug)]
pub enum Error {
    Minreq(minreq::Error),
    WrongResponce(String),
    Ini(ini::Error),
    WrongIniFormat,
}

type Result<T> = std::result::Result<T, Error>;

impl From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Error {
        Error::Minreq(err)
    }
}

impl From<ini::Error> for Error {
    fn from(err: ini::Error) -> Error {
        Error::Ini(err)
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
        Ok(result.to_owned())
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

fn cache_file_name(year: &str, day: &str) -> String {
    format!("cache/cache{}_{}.txt", year, day)
}

pub fn get_input_from_ini_with_year(year: &str, day: &str) -> Result<String> {
    let filename = cache_file_name(year, day);
    std::fs::read_to_string(&filename).or_else(|file_error| {
        log!("Cache not found ({})", file_error);
        let number: String =
            day.chars().filter(|c| c.is_ascii_digit()).collect();
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
        get_input(&url, session).map(|s| {
            if let Err(e) = std::fs::write(filename, &s) {
                log!("{:?}", e);
            }
            s
        })
    })
}

#[derive(Clone, Copy)]
pub struct FunctionHolder {
    pub f: fn(),
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! mod_list {
    ($year: ident, $($day: ident),+) => {
        use once_cell::sync::OnceCell;
        $(pub mod $day;)*
        pub fn task(day: &str) {
            let fn_map = FN_MAP.get_or_init(||std::collections::HashMap::from ([
                $((stringify!($day), crate::common::FunctionHolder {
                        f: || {
                            let year_str = stringify!($year);
                            let day_str = stringify!($day);
                            let input =
                                crate::common::get_input_from_ini_with_mod(year_str, day_str)
                                    .unwrap();
                            let data = $day::parse_input(&input);
                            println!("{} {}", year_str, day_str);
                            println!("Result 1:\n{}", $day::task1(&data));
                            println!("Result 2:\n{}", $day::task2(&data));
                        },
                    }),)*
                ]));
            (fn_map[day].f)()

        }
        static FN_MAP : OnceCell<std::collections::HashMap<&'static str, crate::common::FunctionHolder>> = OnceCell::new();

    }
}

/**
 * Floyd's hare and tortoise algorithm
 * Input: gen - function that generates initial value
 * (maybe by cloning some value)
 * step - function that mutates value, moving it one step forward
 *
 * Return: (lambda, mu)
 * lambda - length of cycle
 * mu - index of the first element in cycle
 */

pub fn floyd_hare_tortoise<Type, Gen, Step>(
    gen: Gen,
    step: Step,
) -> (usize, usize)
where
    Gen: Fn() -> Type,
    Step: Fn(&mut Type),
    Type: PartialEq + Clone,
{
    let mut hare = gen();
    let mut tortoise = gen();
    loop {
        step(&mut hare);
        step(&mut hare);
        step(&mut tortoise);
        if hare == tortoise {
            break;
        }
    }
    let mut mu = 0;
    let mut tortoise = gen();
    while tortoise != hare {
        step(&mut hare);
        step(&mut tortoise);
        mu += 1;
    }
    let mut lam = 1;
    let mut hare = tortoise.clone();
    loop {
        step(&mut hare);
        if hare == tortoise {
            break;
        }
        lam += 1;
    }
    (lam, mu)
}
