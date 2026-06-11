use crate::normalizer;

pub fn parse(text: &str) -> String {
    normalizer::normalize(text)
}
