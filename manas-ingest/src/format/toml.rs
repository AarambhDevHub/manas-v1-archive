use crate::normalizer;

pub fn parse(text: &str) -> String {
    let mut result = String::new();
    match text.parse::<toml::Value>() {
        Ok(table) => {
            flatten_toml(&table, String::new(), &mut result);
        }
        Err(_) => {
            result.push_str(text);
        }
    }
    normalizer::normalize(&result)
}

fn flatten_toml(value: &toml::Value, prefix: String, result: &mut String) {
    match value {
        toml::Value::Table(map) => {
            for (key, val) in map {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                match val {
                    toml::Value::Table(_) | toml::Value::Array(_) => {
                        flatten_toml(val, path, result);
                    }
                    toml::Value::String(s) => {
                        result.push_str(&format!("{} = {}\n", path, s));
                    }
                    other => {
                        result.push_str(&format!("{} = {}\n", path, other));
                    }
                }
            }
        }
        toml::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let path = format!("{}[{}]", prefix, i);
                flatten_toml(val, path, result);
            }
        }
        _ => {
            if !prefix.is_empty() {
                result.push_str(&format!("{} = {}\n", prefix, value));
            }
        }
    }
}
