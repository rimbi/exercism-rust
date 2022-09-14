pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|s| s.iter().collect::<String>())
        .collect()
}
