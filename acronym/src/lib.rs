fn get_acronym_for_word(word: &str) -> String {
    let mut chars = word.chars().filter(|c| c.is_alphabetic());
    if chars.clone().all(|c| c.is_uppercase()) || chars.clone().all(|c| c.is_lowercase()) {
        return chars.nth(0).map(|c| c.to_string()).unwrap_or("".into());
    }
    chars.filter(|c| c.is_uppercase()).collect()
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .trim()
        .split(|c: char| c == ' ' || c == '-')
        .map(|w| get_acronym_for_word(w))
        .collect::<String>()
        .to_uppercase()
}
