use json::JsonValue;

pub fn parse_input(input: &str) -> JsonValue {
    json::parse(input).unwrap()
}

pub fn task1(json: &JsonValue) -> i64 {
    match json {
        JsonValue::Null | JsonValue::String(_)
           | JsonValue::Boolean(_) | JsonValue::Short(_) => 0,
        JsonValue::Number(number) => number.as_fixed_point_i64(0).unwrap_or(0) as i64,
        JsonValue::Object(object) => object.iter().map(|member|task1(member.1)).sum(),
        JsonValue::Array(vec) => vec.iter().map(|element|task1(element)).sum(),
    }
}

pub fn task2(json: &JsonValue) -> i64 {
    let red = JsonValue::String("red".to_string());
    match json {
        JsonValue::Null | JsonValue::String(_)
           | JsonValue::Boolean(_) | JsonValue::Short(_) => 0,
        JsonValue::Number(number) => number.as_fixed_point_i64(0).unwrap_or(0) as i64,
        JsonValue::Object(object) => {
            object.iter()
                  .map(|member|
                    Some(member.1).filter(|&r|r!=&red)
                                  .map(|m|task2(m))
                  )
                  .sum::<Option<i64>>()
                  .unwrap_or(0)
        },
        JsonValue::Array(vec) => vec.iter().map(|element|task2(element)).sum(),
    }
}