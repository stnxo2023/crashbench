//

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            JsonValue::Null       => "null",
            JsonValue::Bool(_)    => "bool",
            JsonValue::Number(_)  => "number",
            JsonValue::Str(_)     => "string",
            JsonValue::Array(_)   => "array",
            JsonValue::Object(_)  => "object",
        }
    }

    pub fn deep_clone(&self) -> JsonValue {
        match self {
            JsonValue::Null        => JsonValue::Null,
            JsonValue::Bool(b)     => JsonValue::Bool(*b),
            JsonValue::Number(n)   => JsonValue::Number(*n),
            JsonValue::Str(s)      => JsonValue::Str(s.clone()),
            JsonValue::Array(arr)  => JsonValue::Array(arr.iter().map(|v| v.deep_clone()).collect()),
            JsonValue::Object(obj) => JsonValue::Object(
                obj.iter().map(|(k, v)| (k.clone(), v.deep_clone())).collect()
            ),
        }
    }

}
