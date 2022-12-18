use serde_json::Value;

pub fn parse_input(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

pub fn task1(json: &Value) -> i64 {
    match json {
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap_or(0),
        Value::Object(object) => {
            object.iter().map(|member| task1(member.1)).sum()
        }
        Value::Array(vec) => vec.iter().map(task1).sum(),
    }
}

pub fn task2(json: &Value) -> i64 {
    let red = Value::String("red".to_string());
    match json {
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap_or(0),
        Value::Object(object) => object
            .iter()
            .map(|member| Some(member.1).filter(|&r| r != &red).map(task2))
            .sum::<Option<i64>>()
            .unwrap_or(0),
        Value::Array(vec) => vec.iter().map(task2).sum(),
    }
}
