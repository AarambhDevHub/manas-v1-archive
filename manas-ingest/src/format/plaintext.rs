use crate::normalizer;

pub fn parse(text: &str) -> String {
    let cleaned = normalizer::strip_control(text);
    cleaned
}
