pub mod floyd_hare_tortoise;
pub use floyd_hare_tortoise::floyd_hare_tortoise;
pub use floyd_hare_tortoise::floyd_hare_tortoise_with_cmp;

pub mod gcd;
pub use gcd::gcd;
pub use gcd::lcm;

pub mod md5;
pub use md5::Md5Hasher;

pub mod network;
pub mod settings;

#[macro_use]
pub mod aoc;

pub use aoc::get_input;
pub use aoc::get_input_with_mod;

#[macro_use]
pub mod error;

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub year: String,
    pub day: String,
}

pub use error::Error;
pub use error::Result;
