use std::collections::HashMap;

const QUOTE: char = '\'';

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != QUOTE)
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches(QUOTE))
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w.to_string()).or_default() += 1;
            acc
        })
}
