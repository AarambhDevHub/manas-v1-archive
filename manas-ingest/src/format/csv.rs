use crate::normalizer;

pub fn parse(text: &str) -> String {
    let mut result = String::new();
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(text.as_bytes());

    let headers: Vec<String> = match reader.headers() {
        Ok(h) => h.iter().map(|s| s.to_string()).collect(),
        Err(_) => {
            result.push_str(text);
            return normalizer::normalize(&result);
        }
    };

    for row in reader.records() {
        match row {
            Ok(record) => {
                for (i, field) in record.iter().enumerate() {
                    if i < headers.len() {
                        result.push_str(&format!("{}: {}; ", headers[i], field));
                    } else {
                        result.push_str(&format!("column{}: {}; ", i, field));
                    }
                }
                result.push('\n');
            }
            Err(_) => continue,
        }
    }

    normalizer::normalize(&result)
}
