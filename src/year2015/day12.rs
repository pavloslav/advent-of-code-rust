use super::aoc::*;

use serde_json::Value;

pub fn parse_input(input: &str) -> Result<Value> {
    Ok(serde_json::from_str(input)?)
}

fn sum_all(json: &Value) -> i64 {
    match json {
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap_or(0),
        Value::Object(object) => {
            object.iter().map(|member| sum_all(member.1)).sum()
        }
        Value::Array(vec) => vec.iter().map(sum_all).sum(),
    }
}

pub fn task1(json: &Value) -> Result<i64> {
    Ok(sum_all(json))
}

fn sum_no_red(json: &Value) -> i64 {
    let red = Value::String("red".to_string());
    match json {
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap_or(0),
        Value::Object(object) => object
            .iter()
            .map(|member| Some(member.1).filter(|&r| r != &red).map(sum_no_red))
            .sum::<Option<i64>>()
            .unwrap_or(0),
        Value::Array(vec) => vec.iter().map(sum_no_red).sum(),
    }
}

pub fn task2(json: &Value) -> Result<i64> {
    Ok(sum_no_red(json))
}
