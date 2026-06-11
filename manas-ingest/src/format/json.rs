use crate::normalizer;

pub fn parse(text: &str) -> String {
    let mut result = String::new();
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(value) => {
            flatten_json(&value, String::new(), &mut result);
        }
        Err(_) => {
            result.push_str(text);
        }
    }
    normalizer::normalize(&result)
}

fn flatten_json(value: &serde_json::Value, prefix: String, result: &mut String) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                match val {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        flatten_json(val, path, result);
                    }
                    serde_json::Value::String(s) => {
                        result.push_str(&format!("{}: {}\n", path, s));
                    }
                    serde_json::Value::Number(n) => {
                        result.push_str(&format!("{}: {}\n", path, n));
                    }
                    serde_json::Value::Bool(b) => {
                        result.push_str(&format!("{}: {}\n", path, b));
                    }
                    serde_json::Value::Null => {
                        result.push_str(&format!("{}: null\n", path));
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let path = if prefix.is_empty() {
                    i.to_string()
                } else {
                    format!("{}[{}]", prefix, i)
                };
                flatten_json(val, path, result);
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.push_str(&format!("{}: {}\n", prefix, value));
            }
        }
    }
}
