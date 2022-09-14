use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let candidate = candidate
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();
    let unique_chars = candidate.chars().collect::<HashSet<_>>();
    unique_chars.len() == candidate.len()
}
