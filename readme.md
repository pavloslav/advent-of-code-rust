My Advent Of Code solutions with a humble framework to avoid code repition.

Put you AOC session cookie and format string into settings.json, like this:

    {
        "session": "YOUR_SESSION_VALUE",
        "link_year": "https://adventofcode.com/{year}/day/{day}/input"
    }

To run, type

    advent_of_code YEAR DAY

e.g.

    advent_of_code 2018 3

What it does:
1. Downloads puzzle input into ./cache folder (if needed)
2. In yearYEAR/src/dayDAY.rs, calls parse_input with the downloaded/cached content
3. Calls task1() and task2() with the results of parsing
4. Outputs results and/or any Result::Err returned from those three functions. Errors are processed with anyhow.

To create a new year:

    cargo new --lib yearYEAR

copy any yearYYYY/src/lib.rs into yearYEAR/src/lib.rs, change the year, and comment out all unimplemented days (i.e. all of them)

add yearYEAR to cargo.toml and src/lib.rs
get a template for puzzle:

    copy ./src/default.rs ./yearYEAR/day1.rs




