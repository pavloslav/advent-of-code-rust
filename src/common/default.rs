/*
 * New definitions with common::Result
 */

use super::super::common::Result;
use super::Error::TaskError;

pub fn parse_input(input: &str) -> Result<&str> {
    input
}

pub fn task1(input: &str) -> Result<usize> {
    unimplemented!();
}

pub fn task2(input: &str) -> Result<usize> {
    unimplemented!();
}

/*
 * Panicing definitions
 */

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(input: &str) -> usize {
    unimplemented!();
}

pub fn task2(input: &str) -> usize {
    unimplemented!();
}
