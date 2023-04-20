pub mod floyd_hare_tortoise;
pub use floyd_hare_tortoise::floyd_hare_tortoise;

pub mod md5;
pub use md5::Md5Hasher;

pub mod settings;
pub mod network;

#[macro_use]
pub mod aoc;

pub use aoc::get_input;
pub use aoc::get_input_with_mod;
pub use aoc::FunctionHolder;
pub use aoc::FunctionHolderPanic;

pub mod error;

pub use error::Error;
pub use error::Result;
